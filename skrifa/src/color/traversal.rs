use std::{cmp::Ordering, collections::HashSet, ops::Range};

use read_fonts::{
    tables::colr::{CompositeMode, Extend},
    types::{BoundingBox, GlyphId, Point},
    ReadError,
};

use super::{
    instance::{
        resolve_clip_box, resolve_paint, ColorStops, ColrInstance, ResolvedColorStop, ResolvedPaint,
    },
    Brush, ColorPainter, ColorStop, PaintCachedColorGlyph, PaintError, Transform,
};

pub(crate) fn get_clipbox_font_units(
    colr_instance: &ColrInstance,
    glyph_id: GlyphId,
) -> Result<Option<BoundingBox<f32>>, ReadError> {
    Ok((*colr_instance)
        .v1_clip_box(glyph_id)?
        .map(|clip_box| resolve_clip_box(colr_instance, &clip_box)))
}

impl From<ResolvedColorStop> for ColorStop {
    fn from(resolved_stop: ResolvedColorStop) -> Self {
        ColorStop {
            offset: resolved_stop.offset,
            alpha: resolved_stop.alpha,
            palette_index: resolved_stop.palette_index,
        }
    }
}

fn make_sorted_resolved_stops(stops: &ColorStops, instance: &ColrInstance) -> Vec<ColorStop> {
    let color_stop_iter = stops.resolve(instance).map(|stop| stop.into());
    let mut collected: Vec<ColorStop> = color_stop_iter.collect();
    collected.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap_or(Ordering::Equal));
    collected
}

struct CollectFillGlyphPainter<'a> {
    brush_transform: Option<Transform>,
    glyph_id: GlyphId,
    parent_painter: &'a mut dyn ColorPainter,
    pub optimization_success: bool,
}

impl<'a> CollectFillGlyphPainter<'a> {
    fn new(parent_painter: &'a mut dyn ColorPainter, glyph_id: GlyphId) -> Self {
        Self {
            brush_transform: None,
            glyph_id,
            parent_painter,
            optimization_success: true,
        }
    }
}

impl<'a> ColorPainter for CollectFillGlyphPainter<'a> {
    fn push_transform(&mut self, transform: Transform) {
        if self.optimization_success {
            match self.brush_transform {
                None => {
                    self.brush_transform = Some(transform);
                }
                Some(ref mut existing_transform) => {
                    *existing_transform *= transform;
                }
            }
        }
    }

    fn pop_transform(&mut self) {
        // Since we only support fill and and transform operations, we need to
        // ignore a popped transform, as this would be called after traversing
        // the graph backup after a fill was performed, but we want to preserve
        // the transform in order to be able to return it.
    }

    fn fill(&mut self, brush: Brush<'_>) {
        if self.optimization_success {
            self.parent_painter
                .fill_glyph(self.glyph_id, self.brush_transform, brush);
        }
    }

    fn push_clip_glyph(&mut self, _: GlyphId) {
        self.optimization_success = false;
    }

    fn push_clip_box(&mut self, _: BoundingBox<f32>) {
        self.optimization_success = false;
    }

    fn pop_clip(&mut self) {
        self.optimization_success = false;
    }

    fn push_layer(&mut self, _: CompositeMode) {
        self.optimization_success = false;
    }

    fn pop_layer(&mut self) {
        self.optimization_success = false;
    }
}

pub(crate) fn traverse_with_callbacks(
    paint: &ResolvedPaint,
    instance: &ColrInstance,
    painter: &mut impl ColorPainter,
    visited_set: &mut HashSet<usize>,
) -> Result<(), PaintError> {
    match paint {
        ResolvedPaint::ColrLayers { range } => {
            for layer_index in range.clone() {
                // Perform cycle detection with paint id here, second part of the tuple.
                let (layer_paint, paint_id) = (*instance).v1_layer(layer_index)?;
                if !visited_set.insert(paint_id) {
                    return Err(PaintError::PaintCycleDetected);
                }
                traverse_with_callbacks(
                    &resolve_paint(instance, &layer_paint)?,
                    instance,
                    painter,
                    visited_set,
                )?;
                visited_set.remove(&paint_id);
            }
            Ok(())
        }
        ResolvedPaint::Solid {
            palette_index,
            alpha,
        } => {
            painter.fill(Brush::Solid {
                palette_index: *palette_index,
                alpha: *alpha,
            });
            Ok(())
        }
        ResolvedPaint::LinearGradient {
            x0,
            y0,
            x1,
            y1,
            x2,
            y2,
            color_stops,
            extend,
        } => {
            let mut p0 = Point::new(*x0, *y0);
            let p1 = Point::new(*x1, *y1);
            let p2 = Point::new(*x2, *y2);

            let dot_product = |a: Point<f32>, b: Point<f32>| -> f32 { a.x * b.x + a.y * b.y };
            let cross_product = |a: Point<f32>, b: Point<f32>| -> f32 { a.x * b.y - a.y * b.x };
            let project_onto = |vector: Point<f32>, point: Point<f32>| -> Point<f32> {
                let length = (point.x * point.x + point.y * point.y).sqrt();
                if length == 0.0 {
                    return Point::default();
                }
                let mut point_normalized = point / length;
                point_normalized *= dot_product(vector, point) / length;
                point_normalized
            };

            let mut resolved_stops = make_sorted_resolved_stops(color_stops, instance);

            // If p0p1 or p0p2 are degenerate probably nothing should be drawn.
            // If p0p1 and p0p2 are parallel then one side is the first color and the other side is
            // the last color, depending on the direction.
            // For now, just use the first color.
            if p1 == p0 || p2 == p0 || cross_product(p1 - p0, p2 - p0) == 0.0 {
                painter.fill(Brush::Solid {
                    palette_index: resolved_stops[0].palette_index,
                    alpha: resolved_stops[0].alpha,
                });
                return Ok(());
            }

            // Follow implementation note in nanoemoji:
            // https://github.com/googlefonts/nanoemoji/blob/0ac6e7bb4d8202db692574d8530a9b643f1b3b3c/src/nanoemoji/svg.py#L188
            // to compute a new gradient end point P3 as the orthogonal
            // projection of the vector from p0 to p1 onto a line perpendicular
            // to line p0p2 and passing through p0.
            let mut perpendicular_to_p2 = p2 - p0;
            perpendicular_to_p2 = Point::new(perpendicular_to_p2.y, -perpendicular_to_p2.x);
            let mut p3 = p0 + project_onto(p1 - p0, perpendicular_to_p2);

            match (
                resolved_stops.first().cloned(),
                resolved_stops.last().cloned(),
            ) {
                (None, _) | (_, None) => {}
                (Some(first_stop), Some(last_stop)) => {
                    let mut color_stop_range = last_stop.offset - first_stop.offset;

                    // Nothing can be drawn for this situation.
                    if color_stop_range == 0.0 && extend != &Extend::Pad {
                        return Ok(());
                    }

                    // In the Pad case, for providing normalized stops in the 0 to 1 range to the client,
                    // insert a color stop at the end. Adding this stop will paint the equivalent gradient,
                    // because: All font-specified color stops are in the same spot, mode is pad, so
                    // everything before this spot is painted with the first color, everything after this spot
                    // is painted with the last color. Not adding this stop would skip the projection below along
                    // the p0-p3 axis and result in specifying non-normalized color stops to the shader.

                    if color_stop_range == 0.0 && extend == &Extend::Pad {
                        let mut extra_stop = last_stop.clone();
                        extra_stop.offset += 1.0;
                        resolved_stops.push(extra_stop);

                        color_stop_range = 1.0;
                    }

                    debug_assert!(color_stop_range != 0.0);

                    if color_stop_range != 1.0 || first_stop.offset != 0.0 {
                        let p0_p3 = p3 - p0;
                        let p0_offset = p0_p3 * first_stop.offset;
                        let p3_offset = p0_p3 * last_stop.offset;

                        p3 = p0 + p3_offset;
                        p0 += p0_offset;

                        let scale_factor = 1.0 / color_stop_range;
                        let start_offset = first_stop.offset;

                        for stop in &mut resolved_stops {
                            stop.offset = (stop.offset - start_offset) * scale_factor;
                        }
                    }

                    painter.fill(Brush::LinearGradient {
                        p0,
                        p1: p3,
                        color_stops: resolved_stops.as_slice(),
                        extend: *extend,
                    });
                }
            }

            Ok(())
        }
        ResolvedPaint::RadialGradient {
            x0,
            y0,
            radius0,
            x1,
            y1,
            radius1,
            color_stops,
            extend,
        } => {
            let mut c0 = Point::new(*x0, *y0);
            let mut c1 = Point::new(*x1, *y1);
            let mut radius0 = *radius0;
            let mut radius1 = *radius1;

            let mut resolved_stops = make_sorted_resolved_stops(color_stops, instance);

            match (
                resolved_stops.first().cloned(),
                resolved_stops.last().cloned(),
            ) {
                (None, _) | (_, None) => {}
                (Some(first_stop), Some(last_stop)) => {
                    let mut color_stop_range = last_stop.offset - first_stop.offset;
                    // Nothing can be drawn for this situation.
                    if color_stop_range == 0.0 && extend != &Extend::Pad {
                        return Ok(());
                    }

                    // In the Pad case, for providing normalized stops in the 0 to 1 range to the client,
                    // insert a color stop at the end. See LinearGradient for more details.

                    if color_stop_range == 0.0 && extend == &Extend::Pad {
                        let mut extra_stop = last_stop.clone();
                        extra_stop.offset += 1.0;
                        resolved_stops.push(extra_stop);
                        color_stop_range = 1.0;
                    }

                    debug_assert!(color_stop_range != 0.0);

                    // If the colorStopRange is 0 at this point, the default behavior of the shader is to
                    // clamp to 1 color stops that are above 1, clamp to 0 for color stops that are below 0,
                    // and repeat the outer color stops at 0 and 1 if the color stops are inside the
                    // range. That will result in the correct rendering.
                    if color_stop_range != 1.0 || first_stop.offset != 0.0 {
                        let c0_to_c1 = c1 - c0;
                        let radius_diff = radius1 - radius0;
                        let scale_factor = 1.0 / color_stop_range;

                        let c0_offset = c0_to_c1 * first_stop.offset;
                        let c1_offset = c0_to_c1 * last_stop.offset;
                        let stops_start_offset = first_stop.offset;

                        // Order of reassignments is important to avoid shadowing variables.
                        c1 = c0 + c1_offset;
                        c0 += c0_offset;
                        radius1 = radius0 + radius_diff * last_stop.offset;
                        radius0 += radius_diff * first_stop.offset;

                        for stop in &mut resolved_stops {
                            stop.offset = (stop.offset - stops_start_offset) * scale_factor;
                        }
                    }

                    painter.fill(Brush::RadialGradient {
                        c0,
                        r0: radius0,
                        c1,
                        r1: radius1,
                        color_stops: resolved_stops.as_slice(),
                        extend: *extend,
                    });
                }
            }
            Ok(())
        }
        ResolvedPaint::SweepGradient {
            center_x,
            center_y,
            start_angle,
            end_angle,
            color_stops,
            extend,
        } => {
            // OpenType 1.9.1 adds a shift to the angle to ease specification of a 0 to 360
            // degree sweep.
            let sweep_angle_to_degrees = |angle| angle * 180.0 + 180.0;

            let start_angle = sweep_angle_to_degrees(start_angle);
            let end_angle = sweep_angle_to_degrees(end_angle);

            // Stop normalization for sweep:

            let sector_angle = end_angle - start_angle;

            let mut resolved_stops = make_sorted_resolved_stops(color_stops, instance);
            if resolved_stops.is_empty() {
                return Ok(());
            }

            match (
                resolved_stops.first().cloned(),
                resolved_stops.last().cloned(),
            ) {
                (None, _) | (_, None) => {}
                (Some(first_stop), Some(last_stop)) => {
                    let mut color_stop_range = last_stop.offset - first_stop.offset;

                    let mut start_angle_scaled = start_angle + sector_angle * first_stop.offset;
                    let mut end_angle_scaled = start_angle + sector_angle * last_stop.offset;

                    let start_offset = first_stop.offset;

                    // Nothing can be drawn for this situation.
                    if color_stop_range == 0.0 && extend != &Extend::Pad {
                        return Ok(());
                    }

                    // In the Pad case, if the color_stop_range is 0 insert a color stop at the end before
                    // normalizing. Adding this stop will paint the equivalent gradient, because: All font
                    // specified color stops are in the same spot, mode is pad, so everything before this
                    // spot is painted with the first color, everything after this spot is painted with
                    // the last color. Not adding this stop will skip the projection and result in
                    // specifying non-normalized color stops to the shader.
                    if color_stop_range == 0.0 && extend == &Extend::Pad {
                        let mut offset_last = last_stop.clone();
                        offset_last.offset += 1.0;
                        resolved_stops.push(offset_last);
                        color_stop_range = 1.0;
                    }

                    debug_assert!(color_stop_range != 0.0);

                    let scale_factor = 1.0 / color_stop_range;

                    for shift_stop in &mut resolved_stops {
                        shift_stop.offset = (shift_stop.offset - start_offset) * scale_factor;
                    }

                    // /* https://docs.microsoft.com/en-us/typography/opentype/spec/colr#sweep-gradients
                    //  * "The angles are expressed in counter-clockwise degrees from
                    //  * the direction of the positive x-axis on the design
                    //  * grid. [...]  The color line progresses from the start angle
                    //  * to the end angle in the counter-clockwise direction;" -
                    //  * Convert angles and stops from counter-clockwise to clockwise
                    //  * for the shader if the gradient is not already reversed due to
                    //  * start angle being larger than end angle. */
                    start_angle_scaled = 360.0 - start_angle_scaled;
                    end_angle_scaled = 360.0 - end_angle_scaled;

                    if start_angle_scaled >= end_angle_scaled {
                        (start_angle_scaled, end_angle_scaled) =
                            (end_angle_scaled, start_angle_scaled);
                        resolved_stops.reverse();
                        for stop in &mut resolved_stops {
                            stop.offset = 1.0 - stop.offset;
                        }
                    }

                    painter.fill(Brush::SweepGradient {
                        c0: Point::new(*center_x, *center_y),
                        start_angle: start_angle_scaled,
                        end_angle: end_angle_scaled,
                        color_stops: resolved_stops.as_slice(),
                        extend: *extend,
                    });
                }
            }
            Ok(())
        }

        ResolvedPaint::Glyph { glyph_id, paint } => {
            let mut optimizer = CollectFillGlyphPainter::new(painter, *glyph_id);
            let mut result = traverse_with_callbacks(
                &resolve_paint(instance, paint)?,
                instance,
                &mut optimizer,
                visited_set,
            );

            // In case the optimization was not successful, just push a clip, and continue unoptimized traversal.
            if !optimizer.optimization_success {
                painter.push_clip_glyph(*glyph_id);
                result = traverse_with_callbacks(
                    &resolve_paint(instance, paint)?,
                    instance,
                    painter,
                    visited_set,
                );
                painter.pop_clip();
            }

            result
        }
        ResolvedPaint::ColrGlyph { glyph_id } => match (*instance).v1_base_glyph(*glyph_id)? {
            Some((base_glyph, base_glyph_paint_id)) => {
                if !visited_set.insert(base_glyph_paint_id) {
                    return Err(PaintError::PaintCycleDetected);
                }

                let draw_result = painter.paint_cached_color_glyph(*glyph_id)?;
                let result = match draw_result {
                    PaintCachedColorGlyph::Ok => Ok(()),
                    PaintCachedColorGlyph::Unimplemented => {
                        let clipbox = get_clipbox_font_units(instance, *glyph_id)?;

                        if let Some(rect) = clipbox {
                            painter.push_clip_box(rect);
                        }

                        let result = traverse_with_callbacks(
                            &resolve_paint(instance, &base_glyph)?,
                            instance,
                            painter,
                            visited_set,
                        );
                        if clipbox.is_some() {
                            painter.pop_clip();
                        }
                        result
                    }
                };
                visited_set.remove(&base_glyph_paint_id);
                result
            }
            None => Err(PaintError::GlyphNotFound(*glyph_id)),
        },
        ResolvedPaint::Transform {
            paint: next_paint, ..
        }
        | ResolvedPaint::Translate {
            paint: next_paint, ..
        }
        | ResolvedPaint::Scale {
            paint: next_paint, ..
        }
        | ResolvedPaint::Rotate {
            paint: next_paint, ..
        }
        | ResolvedPaint::Skew {
            paint: next_paint, ..
        } => {
            painter.push_transform(paint.try_into()?);
            let result = traverse_with_callbacks(
                &resolve_paint(instance, next_paint)?,
                instance,
                painter,
                visited_set,
            );
            painter.pop_transform();
            result
        }
        ResolvedPaint::Composite {
            source_paint,
            mode,
            backdrop_paint,
        } => {
            painter.push_layer(CompositeMode::SrcOver);
            let mut result = traverse_with_callbacks(
                &resolve_paint(instance, backdrop_paint)?,
                instance,
                painter,
                visited_set,
            );
            result?;
            painter.push_layer(*mode);
            result = traverse_with_callbacks(
                &resolve_paint(instance, source_paint)?,
                instance,
                painter,
                visited_set,
            );
            painter.pop_layer();
            painter.pop_layer();
            result
        }
    }
}

pub(crate) fn traverse_v0_range(
    range: &Range<usize>,
    instance: &ColrInstance,
    painter: &mut impl ColorPainter,
) -> Result<(), PaintError> {
    for layer_index in range.clone() {
        let (layer_index, palette_index) = (*instance).v0_layer(layer_index)?;
        painter.fill_glyph(
            layer_index,
            None,
            Brush::Solid {
                palette_index,
                alpha: 1.0,
            },
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use read_fonts::{types::BoundingBox, FontRef, TableProvider};

    use crate::{
        color::{
            instance::ColrInstance, traversal::get_clipbox_font_units,
            traversal_tests::test_glyph_defs::CLIPBOX,
        },
        MetadataProvider,
    };

    #[test]
    fn clipbox_test() {
        let colr_font = font_test_data::COLRV0V1_VARIABLE;
        let font = FontRef::new(colr_font).unwrap();
        let test_glyph_id = font.charmap().map(CLIPBOX[0]).unwrap();
        let upem = font.head().unwrap().units_per_em();

        let base_bounding_box = BoundingBox {
            x_min: 0.0,
            x_max: upem as f32 / 2.0,
            y_min: upem as f32 / 2.0,
            y_max: upem as f32,
        };
        // Fractional value needed to match variation scaling of clipbox.
        const CLIPBOX_SHIFT: f32 = 200.0122;

        macro_rules! test_entry {
            ($axis:literal, $shift:expr, $field:ident) => {
                (
                    $axis,
                    $shift,
                    BoundingBox {
                        $field: base_bounding_box.$field + ($shift),
                        ..base_bounding_box
                    },
                )
            };
        }

        let test_data_expectations = [
            ("", 0.0, base_bounding_box),
            test_entry!("CLXI", CLIPBOX_SHIFT, x_min),
            test_entry!("CLXA", -CLIPBOX_SHIFT, x_max),
            test_entry!("CLYI", CLIPBOX_SHIFT, y_min),
            test_entry!("CLYA", -CLIPBOX_SHIFT, y_max),
        ];

        for axis_test in test_data_expectations {
            let axis_coordinate = (axis_test.0, axis_test.1);
            let location = font.axes().location([axis_coordinate]);
            let color_instance = ColrInstance::new(font.colr().unwrap(), location.coords());
            let clip_box = get_clipbox_font_units(&color_instance, test_glyph_id).unwrap();
            assert!(clip_box.is_some());
            assert!(
                clip_box.unwrap() == axis_test.2,
                "Clip boxes do not match. Actual: {:?}, expected: {:?}",
                clip_box.unwrap(),
                axis_test.2
            );
        }
    }
}
