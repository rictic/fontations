// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The OpenType [Table Directory](https://docs.microsoft.com/en-us/typography/opentype/spec/otff#table-directory)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableDirectory {
    /// 0x00010000 or 0x4F54544F
    pub sfnt_version: u32,
    pub search_range: u16,
    pub entry_selector: u16,
    pub range_shift: u16,
    /// Table records array—one for each top-level table in the font
    pub table_records: Vec<TableRecord>,
}

impl TableDirectory {
    /// Construct a new `TableDirectory`
    pub fn new(
        sfnt_version: u32,
        search_range: u16,
        entry_selector: u16,
        range_shift: u16,
        table_records: Vec<TableRecord>,
    ) -> Self {
        Self {
            sfnt_version,
            search_range,
            entry_selector,
            range_shift,
            table_records: table_records.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for TableDirectory {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.sfnt_version.write_into(writer);
        (array_len(&self.table_records).unwrap() as u16).write_into(writer);
        self.search_range.write_into(writer);
        self.entry_selector.write_into(writer);
        self.range_shift.write_into(writer);
        self.table_records.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("TableDirectory")
    }
}

impl Validate for TableDirectory {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("TableDirectory", |ctx| {
            ctx.in_field("table_records", |ctx| {
                if self.table_records.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.table_records.validate_impl(ctx);
            });
        })
    }
}

/// Record for a table in a font.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TableRecord {
    /// Table identifier.
    pub tag: Tag,
    /// Checksum for the table.
    pub checksum: u32,
    /// Offset from the beginning of the font data.
    pub offset: u32,
    /// Length of the table.
    pub length: u32,
}

impl TableRecord {
    /// Construct a new `TableRecord`
    pub fn new(tag: Tag, checksum: u32, offset: u32, length: u32) -> Self {
        Self {
            tag,
            checksum,
            offset,
            length,
        }
    }
}

impl FontWrite for TableRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.tag.write_into(writer);
        self.checksum.write_into(writer);
        self.offset.write_into(writer);
        self.length.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("TableRecord")
    }
}

impl Validate for TableRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

/// [TTC Header](https://learn.microsoft.com/en-us/typography/opentype/spec/otff#ttc-header)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TTCHeader {
    /// Font Collection ID string: \"ttcf\"
    pub ttc_tag: Tag,
    /// Number of fonts in TTC
    pub num_fonts: u32,
    /// Array of offsets to the TableDirectory for each font from the beginning of the file
    pub table_directory_offsets: Vec<u32>,
    /// Tag indicating that a DSIG table exists, 0x44534947 ('DSIG') (null if no signature)
    pub dsig_tag: Option<u32>,
    /// The length (in bytes) of the DSIG table (null if no signature)
    pub dsig_length: Option<u32>,
    /// The offset (in bytes) of the DSIG table from the beginning of the TTC file (null if no signature)
    pub dsig_offset: Option<u32>,
}

impl Validate for TTCHeader {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("TTCHeader", |ctx| {
            let version: MajorMinor = self.compute_version();
            ctx.in_field("table_directory_offsets", |ctx| {
                if self.table_directory_offsets.len() > (u32::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
            ctx.in_field("dsig_tag", |ctx| {
                if version.compatible((2, 0)) && self.dsig_tag.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("dsig_length", |ctx| {
                if version.compatible((2, 0)) && self.dsig_length.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("dsig_offset", |ctx| {
                if version.compatible((2, 0)) && self.dsig_offset.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
        })
    }
}
