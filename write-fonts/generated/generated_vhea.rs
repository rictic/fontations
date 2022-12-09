// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [vhea](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea) Vertical Header Table
#[derive(Clone, Debug, Default)]
pub struct Vhea {
    /// Typographic ascent.
    pub ascender: FWord,
    /// Typographic descent.
    pub descender: FWord,
    /// Typographic line gap. Negative LineGap values are treated as
    /// zero in some legacy platform implementations.
    pub line_gap: FWord,
    /// Maximum advance height value in 'vmtx' table.
    pub advance_height_max: UfWord,
    /// Minimum top sidebearing value in 'vmtx' table for glyphs with
    /// contours (empty glyphs should be ignored).
    pub min_top_side_bearing: FWord,
    /// Minimum bottom sidebearing value
    pub min_bottom_side_bearing: FWord,
    /// Defined as max( tsb + (yMax-yMin)).
    pub y_max_extent: FWord,
    /// Used to calculate the slope of the cursor (rise/run); 1 for
    /// vertical caret, 0 for horizontal.
    pub caret_slope_rise: i16,
    /// 0 for vertical caret, 1 for horizontal.
    pub caret_slope_run: i16,
    /// The amount by which a slanted highlight on a glyph needs to be
    /// shifted to produce the best appearance. Set to 0 for
    /// non-slanted fonts
    pub caret_offset: i16,
    /// Number of LongMetric entries in 'hmtx'/'vmtx' table
    pub number_of_long_ver_metrics: u16,
}

impl Vhea {
    /// Construct a new `Vhea`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ascender: FWord,
        descender: FWord,
        line_gap: FWord,
        advance_height_max: UfWord,
        min_top_side_bearing: FWord,
        min_bottom_side_bearing: FWord,
        y_max_extent: FWord,
        caret_slope_rise: i16,
        caret_slope_run: i16,
        caret_offset: i16,
        number_of_long_ver_metrics: u16,
    ) -> Self {
        Self {
            ascender,
            descender,
            line_gap,
            advance_height_max,
            min_top_side_bearing,
            min_bottom_side_bearing,
            y_max_extent,
            caret_slope_rise,
            caret_slope_run,
            caret_offset,
            number_of_long_ver_metrics,
        }
    }
}

impl FontWrite for Vhea {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (MajorMinor::VERSION_1_1 as MajorMinor).write_into(writer);
        self.ascender.write_into(writer);
        self.descender.write_into(writer);
        self.line_gap.write_into(writer);
        self.advance_height_max.write_into(writer);
        self.min_top_side_bearing.write_into(writer);
        self.min_bottom_side_bearing.write_into(writer);
        self.y_max_extent.write_into(writer);
        self.caret_slope_rise.write_into(writer);
        self.caret_slope_run.write_into(writer);
        self.caret_offset.write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        (0 as i16).write_into(writer);
        self.number_of_long_ver_metrics.write_into(writer);
    }
}

impl Validate for Vhea {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::vhea::Vhea<'a>> for Vhea {
    fn from_obj_ref(obj: &read_fonts::tables::vhea::Vhea<'a>, _: FontData) -> Self {
        Vhea {
            ascender: obj.ascender(),
            descender: obj.descender(),
            line_gap: obj.line_gap(),
            advance_height_max: obj.advance_height_max(),
            min_top_side_bearing: obj.min_top_side_bearing(),
            min_bottom_side_bearing: obj.min_bottom_side_bearing(),
            y_max_extent: obj.y_max_extent(),
            caret_slope_rise: obj.caret_slope_rise(),
            caret_slope_run: obj.caret_slope_run(),
            caret_offset: obj.caret_offset(),
            number_of_long_ver_metrics: obj.number_of_long_ver_metrics(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::vhea::Vhea<'a>> for Vhea {}

impl<'a> FontRead<'a> for Vhea {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::vhea::Vhea as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}
