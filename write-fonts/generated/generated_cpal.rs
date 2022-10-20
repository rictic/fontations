// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [CPAL (Color Palette Table)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-table-header) table
#[derive(Clone, Debug)]
pub struct Cpal {
    /// Table version number (=0).
    pub version: u16,
    /// Number of palette entries in each palette.
    pub num_palette_entries: u16,
    /// Number of palettes in the table.
    pub num_palettes: u16,
    /// Total number of color records, combined for all palettes.
    pub num_color_records: u16,
    /// Offset from the beginning of CPAL table to the first
    /// ColorRecord.
    pub color_records_array_offset: NullableOffsetMarker<ColorRecordArray, WIDTH_32>,
    /// Index of each palette’s first color record in the combined
    /// color record array.
    pub color_record_indices: Vec<u16>,
    /// Offset from the beginning of CPAL table to the Palette Types
    /// Array. Set to 0 if no array is provided.
    pub palette_types_array_offset: NullableOffsetMarker<PaletteTypeArray, WIDTH_32>,
    /// Offset from the beginning of CPAL table to the Palette Labels
    /// Array. Set to 0 if no array is provided.
    pub palette_labels_array_offset: NullableOffsetMarker<PaletteLabelArray, WIDTH_32>,
    /// Offset from the beginning of CPAL table to the Palette Entry
    /// Labels Array. Set to 0 if no array is provided.
    pub palette_entry_labels_array_offset: NullableOffsetMarker<PaletteLabelEntryArray, WIDTH_32>,
}

impl FontWrite for Cpal {
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.version;
        version.write_into(writer);
        self.num_palette_entries.write_into(writer);
        self.num_palettes.write_into(writer);
        self.num_color_records.write_into(writer);
        self.color_records_array_offset.write_into(writer);
        self.color_record_indices.write_into(writer);
        version
            .compatible(1)
            .then(|| self.palette_types_array_offset.write_into(writer));
        version
            .compatible(1)
            .then(|| self.palette_labels_array_offset.write_into(writer));
        version
            .compatible(1)
            .then(|| self.palette_entry_labels_array_offset.write_into(writer));
    }
}

impl Validate for Cpal {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Cpal", |ctx| {
            ctx.in_field("color_records_array_offset", |ctx| {
                self.color_records_array_offset.validate_impl(ctx);
            });
            ctx.in_field("color_record_indices", |ctx| {
                if self.color_record_indices.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
            ctx.in_field("palette_types_array_offset", |ctx| {
                self.palette_types_array_offset.validate_impl(ctx);
            });
            ctx.in_field("palette_labels_array_offset", |ctx| {
                self.palette_labels_array_offset.validate_impl(ctx);
            });
            ctx.in_field("palette_entry_labels_array_offset", |ctx| {
                self.palette_entry_labels_array_offset.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::tables::cpal::Cpal<'a>> for Cpal {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::Cpal<'a>, _: FontData) -> Self {
        Cpal {
            version: obj.version(),
            num_palette_entries: obj.num_palette_entries(),
            num_palettes: obj.num_palettes(),
            num_color_records: obj.num_color_records(),
            color_records_array_offset: obj.color_records_array().into(),
            color_record_indices: obj.color_record_indices().iter().map(|x| x.get()).collect(),
            palette_types_array_offset: obj.palette_types_array().into(),
            palette_labels_array_offset: obj.palette_labels_array().into(),
            palette_entry_labels_array_offset: obj.palette_entry_labels_array().into(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::tables::cpal::Cpal<'a>> for Cpal {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for Cpal {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::cpal::Cpal as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [CPAL (Palette Type Array)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-type-array) record
#[derive(Clone, Debug)]
pub struct ColorRecordArray {
    /// Array of 32-bit flag fields that describe properties of each
    /// palette. See below for details.
    pub color_records: Vec<ColorRecord>,
}

impl FontWrite for ColorRecordArray {
    fn write_into(&self, writer: &mut TableWriter) {
        self.color_records.write_into(writer);
    }
}

impl Validate for ColorRecordArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("ColorRecordArray", |ctx| {
            ctx.in_field("color_records", |ctx| {
                if self.color_records.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.color_records.validate_impl(ctx);
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::tables::cpal::ColorRecordArray<'a>> for ColorRecordArray {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::ColorRecordArray<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        ColorRecordArray {
            color_records: obj
                .color_records()
                .iter()
                .map(|x| FromObjRef::from_obj_ref(x, offset_data))
                .collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::tables::cpal::ColorRecordArray<'a>> for ColorRecordArray {}

/// [CPAL (Color Record)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entries-and-color-records) record
#[derive(Clone, Debug)]
pub struct ColorRecord {
    /// Blue value (B0).
    pub blue: u8,
    /// Green value (B1).
    pub green: u8,
    ///     Red value (B2).
    pub red: u8,
    /// Alpha value (B3).
    pub alpha: u8,
}

impl FontWrite for ColorRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.blue.write_into(writer);
        self.green.write_into(writer);
        self.red.write_into(writer);
        self.alpha.write_into(writer);
    }
}

impl Validate for ColorRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

#[cfg(feature = "parsing")]
impl FromObjRef<read_fonts::tables::cpal::ColorRecord> for ColorRecord {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::ColorRecord, _: FontData) -> Self {
        ColorRecord {
            blue: obj.blue(),
            green: obj.green(),
            red: obj.red(),
            alpha: obj.alpha(),
        }
    }
}

/// [CPAL (Palette Type Array)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-type-array) record
#[derive(Clone, Debug)]
pub struct PaletteTypeArray {
    /// Array of 32-bit flag fields that describe properties of each
    /// palette. See below for details.
    pub palette_types: Vec<u32>,
}

impl FontWrite for PaletteTypeArray {
    fn write_into(&self, writer: &mut TableWriter) {
        self.palette_types.write_into(writer);
    }
}

impl Validate for PaletteTypeArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PaletteTypeArray", |ctx| {
            ctx.in_field("palette_types", |ctx| {
                if self.palette_types.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::tables::cpal::PaletteTypeArray<'a>> for PaletteTypeArray {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::PaletteTypeArray<'a>, _: FontData) -> Self {
        PaletteTypeArray {
            palette_types: obj.palette_types().iter().map(|x| x.get()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::tables::cpal::PaletteTypeArray<'a>> for PaletteTypeArray {}

/// [CPAL (Palette Label Array)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-labels-array) record
#[derive(Clone, Debug)]
pub struct PaletteLabelArray {
    /// Array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with
    /// each palette. Use 0xFFFF if no name ID is provided for a
    /// palette.
    pub palette_labels: Vec<u16>,
}

impl FontWrite for PaletteLabelArray {
    fn write_into(&self, writer: &mut TableWriter) {
        self.palette_labels.write_into(writer);
    }
}

impl Validate for PaletteLabelArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PaletteLabelArray", |ctx| {
            ctx.in_field("palette_labels", |ctx| {
                if self.palette_labels.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::tables::cpal::PaletteLabelArray<'a>> for PaletteLabelArray {
    fn from_obj_ref(obj: &read_fonts::tables::cpal::PaletteLabelArray<'a>, _: FontData) -> Self {
        PaletteLabelArray {
            palette_labels: obj.palette_labels().iter().map(|x| x.get()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::tables::cpal::PaletteLabelArray<'a>> for PaletteLabelArray {}

/// [CPAL (Palette Label Entry Array)](https://learn.microsoft.com/en-us/typography/opentype/spec/cpal#palette-entry-label-array) record
#[derive(Clone, Debug)]
pub struct PaletteLabelEntryArray {
    /// Array of 'name' table IDs (typically in the font-specific name
    /// ID range) that specify user interface strings associated with
    /// each palette entry, e.g. “Outline”, “Fill”. This set of
    /// palette entry labels applies to all palettes in the font. Use
    /// 0xFFFF if no name ID is provided for a palette entry.
    pub palette_entry_labels: Vec<u16>,
}

impl FontWrite for PaletteLabelEntryArray {
    fn write_into(&self, writer: &mut TableWriter) {
        self.palette_entry_labels.write_into(writer);
    }
}

impl Validate for PaletteLabelEntryArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PaletteLabelEntryArray", |ctx| {
            ctx.in_field("palette_entry_labels", |ctx| {
                if self.palette_entry_labels.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::tables::cpal::PaletteLabelEntryArray<'a>>
    for PaletteLabelEntryArray
{
    fn from_obj_ref(
        obj: &read_fonts::tables::cpal::PaletteLabelEntryArray<'a>,
        _: FontData,
    ) -> Self {
        PaletteLabelEntryArray {
            palette_entry_labels: obj.palette_entry_labels().iter().map(|x| x.get()).collect(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::tables::cpal::PaletteLabelEntryArray<'a>>
    for PaletteLabelEntryArray
{
}