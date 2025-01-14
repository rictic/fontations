//! Loading, scaling and hinting of glyph outlines.
//!
//! This module provides support for retrieving (optionally scaled and hinted)
//! glyph outlines in the form of vector paths.
//!
//! # Drawing a glyph
//!
//! Generating SVG [path commands](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#path_commands)
//! for a character (this assumes a local variable `font` of type [`FontRef`](crate::FontRef)):
//!
//! ```rust
//! use skrifa::{
//!     instance::{LocationRef, Size},
//!     outline::{DrawSettings, OutlinePen},
//!     FontRef, MetadataProvider,
//! };
//!
//! # fn wrapper(font: FontRef) {
//! // First, grab the set of outline glyphs from the font.
//! let outlines = font.outline_glyphs();
//!
//! // Find the glyph identifier for our character.
//! let glyph_id = font.charmap().map('Q').unwrap();
//!
//! // Grab the outline glyph.
//! let glyph = outlines.get(glyph_id).unwrap();
//!
//! // Define how we want the glyph to be drawn. This creates
//! // settings for an instance without hinting at a size of
//! // 16px with no variations applied.
//! let settings = DrawSettings::unhinted(Size::new(16.0), LocationRef::default());
//!
//! // Alternatively, we can apply variations like so:
//! let var_location = font.axes().location(&[("wght", 650.0), ("wdth", 100.0)]);
//! let settings = DrawSettings::unhinted(Size::new(16.0), &var_location);
//!
//! // At this point, we need a "sink" to receive the resulting path. This
//! // is done by creating an implementation of the OutlinePen trait.
//!
//! // Let's make one that generates SVG path data.
//! #[derive(Default)]
//! struct SvgPath(String);
//!
//! // Implement the OutlinePen trait for this type. This emits the appropriate
//! // SVG path commands for each element type.
//! impl OutlinePen for SvgPath {
//!     fn move_to(&mut self, x: f32, y: f32) {
//!         self.0.push_str(&format!("M{x:.1},{y:.1} "));
//!     }
//!
//!     fn line_to(&mut self, x: f32, y: f32) {
//!         self.0.push_str(&format!("L{x:.1},{y:.1} "));
//!     }
//!
//!     fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
//!         self.0
//!             .push_str(&format!("Q{cx0:.1},{cy0:.1} {x:.1},{y:.1} "));
//!     }
//!
//!     fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
//!         self.0.push_str(&format!(
//!             "C{cx0:.1},{cy0:.1} {cx1:.1},{cy1:.1} {x:.1},{y:.1} "
//!         ));
//!     }
//!
//!     fn close(&mut self) {
//!         self.0.push_str("Z ");
//!     }
//! }
//! // Now, construct an instance of our pen.
//! let mut svg_path = SvgPath::default();
//!
//! // And draw the glyph!
//! glyph.draw(settings, &mut svg_path).unwrap();
//!
//! // See what we've drawn.
//! println!("{}", svg_path.0);
//! # }
//! ```

mod cff;
mod embedded_hinting;
mod glyf;

pub mod error;

use read_fonts::{types::GlyphId, TableProvider};

pub use embedded_hinting::{EmbeddedHintingInstance, HintingMode, LcdLayout};
#[doc(inline)]
pub use error::DrawError;

pub use read_fonts::types::Pen as OutlinePen;

use super::{
    instance::{LocationRef, NormalizedCoord, Size},
    GLYF_COMPOSITE_RECURSION_LIMIT,
};

/// Source format for an outline glyph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum OutlineGlyphFormat {
    /// TrueType outlines sourced from the `glyf` table.
    Glyf,
    /// PostScript outlines sourced from the `CFF` table.
    Cff,
    /// PostScript outlines sourced from the `CFF2` table.
    Cff2,
}

/// Specifies the hinting strategy for memory size calculations.
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Hinting {
    /// Hinting is disabled.
    #[default]
    None,
    /// Application of hints that are embedded in the font.
    ///
    /// For TrueType, these are bytecode instructions associated with each
    /// glyph outline. For PostScript (CFF/CFF2), these are stem hints
    /// encoded in the character string.
    Embedded,
}

/// Information and adjusted metrics generated while drawing an outline glyph.
///
/// When applying hints to a TrueType glyph, the outline may be shifted in
/// the horizontal direction, affecting the left side bearing and advance width
/// of the glyph. This captures those metrics.
#[derive(Copy, Clone, Default, Debug)]
pub struct AdjustedMetrics {
    /// True if the underlying glyph contains flags indicating the
    /// presence of overlapping contours or components.
    pub has_overlaps: bool,
    /// If present, an adjusted left side bearing value generated by the
    /// scaler.
    ///
    /// This is equivalent to the `horiBearingX` value in
    /// [`FT_Glyph_Metrics`](https://freetype.org/freetype2/docs/reference/ft2-glyph_retrieval.html#ft_glyph_metrics).
    pub lsb: Option<f32>,
    /// If present, an adjusted advance width value generated by the
    /// scaler.
    ///
    /// This is equivalent to the `advance.x` value in
    /// [`FT_GlyphSlotRec`](https://freetype.org/freetype2/docs/reference/ft2-glyph_retrieval.html#ft_glyphslotrec).
    pub advance_width: Option<f32>,
}

/// Options that define how a [glyph](OutlineGlyph) is drawn to a
/// [pen](OutlinePen).
pub struct DrawSettings<'a> {
    instance: DrawInstance<'a>,
    memory: Option<&'a mut [u8]>,
}

impl<'a> DrawSettings<'a> {
    /// Creates settings for an unhinted draw operation with the given size and
    /// location in variation space.
    pub fn unhinted(size: Size, location: impl Into<LocationRef<'a>>) -> Self {
        Self {
            instance: DrawInstance::Unhinted(size, location.into()),
            memory: None,
        }
    }

    /// Creates settings for a hinted draw operation using embedded hinting.
    ///
    /// The font size, location in variation space and hinting mode are
    /// defined by the current configuration of the given hinting instance.
    pub fn embedded_hinting(instance: &'a EmbeddedHintingInstance) -> Self {
        Self {
            instance: DrawInstance::EmbeddedHinted(instance),
            memory: None,
        }
    }

    /// Builder method to associate a user memory buffer to be used for
    /// temporary allocations during drawing.
    ///
    /// The required size of this buffer can be computed using the
    /// [`OutlineGlyph::draw_memory_size`] method.
    ///
    /// If not provided, any necessary memory will be allocated internally.
    pub fn with_memory(mut self, memory: Option<&'a mut [u8]>) -> Self {
        self.memory = memory;
        self
    }
}

enum DrawInstance<'a> {
    Unhinted(Size, LocationRef<'a>),
    EmbeddedHinted(&'a EmbeddedHintingInstance),
}

impl<'a, L> From<(Size, L)> for DrawSettings<'a>
where
    L: Into<LocationRef<'a>>,
{
    fn from(value: (Size, L)) -> Self {
        DrawSettings::unhinted(value.0, value.1.into())
    }
}

impl From<Size> for DrawSettings<'_> {
    fn from(value: Size) -> Self {
        DrawSettings::unhinted(value, LocationRef::default())
    }
}

impl<'a> From<&'a EmbeddedHintingInstance> for DrawSettings<'a> {
    fn from(value: &'a EmbeddedHintingInstance) -> Self {
        DrawSettings::embedded_hinting(value)
    }
}

/// A scalable glyph outline.
///
/// This can be sourced from the [`glyf`](https://learn.microsoft.com/en-us/typography/opentype/spec/glyf),
/// [`CFF`](https://learn.microsoft.com/en-us/typography/opentype/spec/cff) or
/// [`CFF2`](https://learn.microsoft.com/en-us/typography/opentype/spec/cff2)
/// tables. Use the [`format`](OutlineGlyph::format) method to determine which
/// was chosen for this glyph.
#[derive(Clone)]
pub struct OutlineGlyph<'a> {
    kind: OutlineKind<'a>,
}

impl<'a> OutlineGlyph<'a> {
    /// Returns the underlying source format for this outline.
    pub fn format(&self) -> OutlineGlyphFormat {
        match &self.kind {
            OutlineKind::Glyf(..) => OutlineGlyphFormat::Glyf,
            OutlineKind::Cff(cff, ..) => {
                if cff.is_cff2() {
                    OutlineGlyphFormat::Cff2
                } else {
                    OutlineGlyphFormat::Cff
                }
            }
        }
    }

    /// Returns a value indicating if the outline may contain overlapping
    /// contours or components.
    ///
    /// For CFF outlines, returns `None` since this information is unavailable.
    pub fn has_overlaps(&self) -> Option<bool> {
        match &self.kind {
            OutlineKind::Glyf(_, outline) => Some(outline.has_overlaps),
            _ => None,
        }
    }

    /// Returns a value indicating whether the outline has hinting
    /// instructions.
    ///
    /// For CFF outlines, returns `None` since this is unknown prior
    /// to loading the outline.
    pub fn has_hinting(&self) -> Option<bool> {
        match &self.kind {
            OutlineKind::Glyf(_, outline) => Some(outline.has_hinting),
            _ => None,
        }
    }

    /// Returns the size (in bytes) of the temporary memory required to draw
    /// this outline.
    ///
    /// This is used to compute the size of the memory buffer required for the
    /// [`DrawSettings::with_memory`] method.
    ///
    /// The `hinting` parameter determines which hinting method, if any, will
    /// be used for drawing which has an effect on memory requirements.
    ///
    /// The appropriate hinting types are as follows:
    ///
    /// | For draw settings                  | Use hinting           |
    /// |------------------------------------|-----------------------|
    /// | [`DrawSettings::unhinted`]         | [`Hinting::None`]     |
    /// | [`DrawSettings::embedded_hinting`] | [`Hinting::Embedded`] |
    pub fn draw_memory_size(&self, hinting: Hinting) -> usize {
        match &self.kind {
            OutlineKind::Glyf(_, outline) => outline.required_buffer_size(hinting),
            _ => 0,
        }
    }

    /// Draws the outline glyph with the given settings and emits the resulting
    /// path commands to the specified pen.
    pub fn draw<'s>(
        &self,
        settings: impl Into<DrawSettings<'a>>,
        pen: &mut impl OutlinePen,
    ) -> Result<AdjustedMetrics, DrawError> {
        let settings = settings.into();
        match settings.instance {
            DrawInstance::Unhinted(size, location) => {
                self.draw_unhinted(size, location, settings.memory, pen)
            }
            DrawInstance::EmbeddedHinted(hinter) => hinter.draw(self, settings.memory, pen),
        }
    }

    fn draw_unhinted(
        &self,
        size: Size,
        location: impl Into<LocationRef<'a>>,
        memory: Option<&mut [u8]>,
        pen: &mut impl OutlinePen,
    ) -> Result<AdjustedMetrics, DrawError> {
        let ppem = size.ppem();
        let coords = location.into().coords();
        match &self.kind {
            OutlineKind::Glyf(glyf, outline) => {
                with_glyf_memory(outline, Hinting::None, memory, |buf| {
                    let mem = outline
                        .memory_from_buffer(buf, Hinting::None)
                        .ok_or(DrawError::InsufficientMemory)?;
                    let scaled_outline = glyf.draw(mem, outline, ppem, coords)?;
                    scaled_outline.to_path(pen)?;
                    Ok(AdjustedMetrics {
                        has_overlaps: outline.has_overlaps,
                        lsb: Some(scaled_outline.adjusted_lsb().to_f32()),
                        advance_width: Some(scaled_outline.adjusted_advance_width().to_f32()),
                    })
                })
            }
            OutlineKind::Cff(cff, glyph_id, subfont_ix) => {
                let subfont = cff.subfont(*subfont_ix, ppem, coords)?;
                cff.draw(&subfont, *glyph_id, coords, false, pen)?;
                Ok(AdjustedMetrics::default())
            }
        }
    }
}

#[derive(Clone)]
enum OutlineKind<'a> {
    Glyf(glyf::Outlines<'a>, glyf::Outline<'a>),
    // Third field is subfont index
    Cff(cff::Outlines<'a>, GlyphId, u32),
}

/// Collection of scalable glyph outlines.
#[derive(Clone)]
pub struct OutlineGlyphCollection<'a> {
    kind: OutlineCollectionKind<'a>,
}

impl<'a> OutlineGlyphCollection<'a> {
    /// Creates a new outline collection for the given font.
    pub fn new(font: &impl TableProvider<'a>) -> Self {
        let kind = if let Some(glyf) = glyf::Outlines::new(font) {
            OutlineCollectionKind::Glyf(glyf)
        } else if let Ok(cff) = cff::Outlines::new(font) {
            OutlineCollectionKind::Cff(cff)
        } else {
            OutlineCollectionKind::None
        };
        Self { kind }
    }

    /// Creates a new outline collection for the given font and outline
    /// format.
    ///
    /// Returns `None` if the font does not contain outlines in the requested
    /// format.
    pub fn with_format(font: &impl TableProvider<'a>, format: OutlineGlyphFormat) -> Option<Self> {
        let kind = match format {
            OutlineGlyphFormat::Glyf => OutlineCollectionKind::Glyf(glyf::Outlines::new(font)?),
            OutlineGlyphFormat::Cff => {
                let upem = font.head().ok()?.units_per_em();
                OutlineCollectionKind::Cff(cff::Outlines::from_cff(font.cff().ok()?, 0, upem).ok()?)
            }
            OutlineGlyphFormat::Cff2 => {
                let upem = font.head().ok()?.units_per_em();
                OutlineCollectionKind::Cff(cff::Outlines::from_cff2(font.cff2().ok()?, upem).ok()?)
            }
        };
        Some(Self { kind })
    }

    /// Returns the underlying format of the source outline tables.
    pub fn format(&self) -> Option<OutlineGlyphFormat> {
        match &self.kind {
            OutlineCollectionKind::Glyf(..) => Some(OutlineGlyphFormat::Glyf),
            OutlineCollectionKind::Cff(cff) => cff
                .is_cff2()
                .then_some(OutlineGlyphFormat::Cff2)
                .or(Some(OutlineGlyphFormat::Cff)),
            _ => None,
        }
    }

    /// Returns the outline for the given glyph identifier.
    pub fn get(&self, glyph_id: GlyphId) -> Option<OutlineGlyph<'a>> {
        match &self.kind {
            OutlineCollectionKind::None => None,
            OutlineCollectionKind::Glyf(glyf) => Some(OutlineGlyph {
                kind: OutlineKind::Glyf(glyf.clone(), glyf.outline(glyph_id).ok()?),
            }),
            OutlineCollectionKind::Cff(cff) => Some(OutlineGlyph {
                kind: OutlineKind::Cff(cff.clone(), glyph_id, cff.subfont_index(glyph_id)),
            }),
        }
    }

    /// Returns an iterator over all of the outline glyphs in the collection.
    pub fn iter(&self) -> impl Iterator<Item = (GlyphId, OutlineGlyph<'a>)> + 'a + Clone {
        let len = match &self.kind {
            OutlineCollectionKind::Glyf(glyf) => glyf.glyph_count(),
            OutlineCollectionKind::Cff(cff) => cff.glyph_count(),
            _ => 0,
        } as u16;
        let copy = self.clone();
        (0..len).filter_map(move |gid| {
            let gid = GlyphId::new(gid);
            let glyph = copy.get(gid)?;
            Some((gid, glyph))
        })
    }
}

#[derive(Clone)]
enum OutlineCollectionKind<'a> {
    None,
    Glyf(glyf::Outlines<'a>),
    Cff(cff::Outlines<'a>),
}

/// Arbitrarily chosen smallish size for stack allocation to avoid the heap
/// when possible while drawing glyf outlines.
///
/// Upcoming work on TrueType hinting will likely adjust this to use bucketed
/// sizes based on actual data captured from fonts.
const GLYF_DRAW_STACK_BUFFER_SIZE: usize = 4096;

/// Invokes the callback with a memory buffer suitable for drawing
/// the given TrueType outline.
pub(super) fn with_glyf_memory<R>(
    outline: &glyf::Outline,
    hinting: Hinting,
    memory: Option<&mut [u8]>,
    mut f: impl FnMut(&mut [u8]) -> R,
) -> R {
    // Wrap in a function and prevent inlining to avoid stack allocation
    // and zeroing if we don't take this code path.
    #[inline(never)]
    fn stack_mem<R>(mut f: impl FnMut(&mut [u8]) -> R) -> R {
        f(&mut [0u8; GLYF_DRAW_STACK_BUFFER_SIZE])
    }
    match memory {
        Some(buf) => f(buf),
        None => {
            let buf_size = outline.required_buffer_size(hinting);
            if buf_size <= GLYF_DRAW_STACK_BUFFER_SIZE {
                stack_mem(f)
            } else {
                f(&mut vec![0u8; buf_size])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetadataProvider;
    use read_fonts::{scaler_test, types::GlyphId, FontRef, TableProvider};

    #[test]
    fn outline_glyph_formats() {
        let font_format_pairs = [
            (font_test_data::VAZIRMATN_VAR, OutlineGlyphFormat::Glyf),
            (
                font_test_data::CANTARELL_VF_TRIMMED,
                OutlineGlyphFormat::Cff2,
            ),
            (
                font_test_data::NOTO_SERIF_DISPLAY_TRIMMED,
                OutlineGlyphFormat::Cff,
            ),
            (font_test_data::COLRV0V1_VARIABLE, OutlineGlyphFormat::Glyf),
        ];
        for (font_data, format) in font_format_pairs {
            assert_eq!(
                FontRef::new(font_data).unwrap().outline_glyphs().format(),
                Some(format)
            );
        }
    }

    #[test]
    fn vazirmatin_var() {
        compare_glyphs(
            font_test_data::VAZIRMATN_VAR,
            font_test_data::VAZIRMATN_VAR_GLYPHS,
        );
    }

    #[test]
    fn cantarell_vf() {
        compare_glyphs(
            font_test_data::CANTARELL_VF_TRIMMED,
            font_test_data::CANTARELL_VF_TRIMMED_GLYPHS,
        );
    }

    #[test]
    fn noto_serif_display() {
        compare_glyphs(
            font_test_data::NOTO_SERIF_DISPLAY_TRIMMED,
            font_test_data::NOTO_SERIF_DISPLAY_TRIMMED_GLYPHS,
        );
    }

    #[test]
    fn overlap_flags() {
        let font = FontRef::new(font_test_data::VAZIRMATN_VAR).unwrap();
        let outlines = font.outline_glyphs();
        let glyph_count = font.maxp().unwrap().num_glyphs();
        // GID 2 is a composite glyph with the overlap bit on a component
        // GID 3 is a simple glyph with the overlap bit on the first flag
        let expected_gids_with_overlap = vec![2, 3];
        assert_eq!(
            expected_gids_with_overlap,
            (0..glyph_count)
                .filter(
                    |gid| outlines.get(GlyphId::new(*gid)).unwrap().has_overlaps() == Some(true)
                )
                .collect::<Vec<_>>()
        );
    }

    fn compare_glyphs(font_data: &[u8], expected_outlines: &str) {
        let font = FontRef::new(font_data).unwrap();
        let expected_outlines = scaler_test::parse_glyph_outlines(expected_outlines);
        let mut path = scaler_test::Path::default();
        for expected_outline in &expected_outlines {
            if expected_outline.size == 0.0 && !expected_outline.coords.is_empty() {
                continue;
            }
            let size = if expected_outline.size != 0.0 {
                Size::new(expected_outline.size)
            } else {
                Size::unscaled()
            };
            path.elements.clear();
            font.outline_glyphs()
                .get(expected_outline.glyph_id)
                .unwrap()
                .draw(
                    DrawSettings::unhinted(size, expected_outline.coords.as_slice()),
                    &mut path,
                )
                .unwrap();
            if path.elements != expected_outline.path {
                panic!(
                    "mismatch in glyph path for id {} (size: {}, coords: {:?}): path: {:?} expected_path: {:?}",
                    expected_outline.glyph_id,
                    expected_outline.size,
                    expected_outline.coords,
                    &path.elements,
                    &expected_outline.path
                );
            }
        }
    }
}
