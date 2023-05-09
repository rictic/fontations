// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// An array of variable-sized objects in a `CFF` table.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Index1 {
    /// Number of objects stored in INDEX.
    pub count: u16,
    /// Object array element size.
    pub off_size: u8,
    /// Bytes containing `count + 1` offsets each of `off_size`.
    pub offsets: Vec<u8>,
    /// Array containing the object data.
    pub data: Vec<u8>,
}

impl Index1 {
    /// Construct a new `Index1`
    pub fn new(count: u16, off_size: u8, offsets: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            count,
            off_size,
            offsets: offsets.into_iter().map(Into::into).collect(),
            data: data.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for Index1 {
    fn write_into(&self, writer: &mut TableWriter) {
        self.count.write_into(writer);
        self.off_size.write_into(writer);
        self.offsets.write_into(writer);
        self.data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("Index1")
    }
}

impl Validate for Index1 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::postscript::Index1<'a>> for Index1 {
    fn from_obj_ref(obj: &read_fonts::tables::postscript::Index1<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Index1 {
            count: obj.count(),
            off_size: obj.off_size(),
            offsets: obj.offsets().to_owned_obj(offset_data),
            data: obj.data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::postscript::Index1<'a>> for Index1 {}

impl<'a> FontRead<'a> for Index1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::postscript::Index1 as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// An array of variable-sized objects in a `CFF2` table.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Index2 {
    /// Number of objects stored in INDEX.
    pub count: u32,
    /// Object array element size.
    pub off_size: u8,
    /// Bytes containing `count + 1` offsets each of `off_size`.
    pub offsets: Vec<u8>,
    /// Array containing the object data.
    pub data: Vec<u8>,
}

impl Index2 {
    /// Construct a new `Index2`
    pub fn new(count: u32, off_size: u8, offsets: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            count,
            off_size,
            offsets: offsets.into_iter().map(Into::into).collect(),
            data: data.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for Index2 {
    fn write_into(&self, writer: &mut TableWriter) {
        self.count.write_into(writer);
        self.off_size.write_into(writer);
        self.offsets.write_into(writer);
        self.data.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("Index2")
    }
}

impl Validate for Index2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::postscript::Index2<'a>> for Index2 {
    fn from_obj_ref(obj: &read_fonts::tables::postscript::Index2<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        Index2 {
            count: obj.count(),
            off_size: obj.off_size(),
            offsets: obj.offsets().to_owned_obj(offset_data),
            data: obj.data().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::postscript::Index2<'a>> for Index2 {}

impl<'a> FontRead<'a> for Index2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::postscript::Index2 as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// Associates a glyph identifier with a Font DICT.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FdSelect {
    Format0(FdSelectFormat0),
    Format3(FdSelectFormat3),
    Format4(FdSelectFormat4),
}

impl FdSelect {
    /// Construct a new `FdSelectFormat0` subtable
    pub fn format_0(fds: Vec<u8>) -> Self {
        Self::Format0(FdSelectFormat0::new(fds))
    }

    /// Construct a new `FdSelectFormat3` subtable
    pub fn format_3(ranges: Vec<FdSelectRange3>, sentinel: u16) -> Self {
        Self::Format3(FdSelectFormat3::new(ranges, sentinel))
    }

    /// Construct a new `FdSelectFormat4` subtable
    pub fn format_4(ranges: Vec<FdSelectRange4>, sentinel: u32) -> Self {
        Self::Format4(FdSelectFormat4::new(ranges, sentinel))
    }
}

impl Default for FdSelect {
    fn default() -> Self {
        Self::Format0(Default::default())
    }
}

impl FontWrite for FdSelect {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format0(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
            Self::Format4(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format0(item) => item.table_type(),
            Self::Format3(item) => item.table_type(),
            Self::Format4(item) => item.table_type(),
        }
    }
}

impl Validate for FdSelect {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format0(item) => item.validate_impl(ctx),
            Self::Format3(item) => item.validate_impl(ctx),
            Self::Format4(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::postscript::FdSelect<'_>> for FdSelect {
    fn from_obj_ref(obj: &read_fonts::tables::postscript::FdSelect, _: FontData) -> Self {
        use read_fonts::tables::postscript::FdSelect as ObjRefType;
        match obj {
            ObjRefType::Format0(item) => FdSelect::Format0(item.to_owned_table()),
            ObjRefType::Format3(item) => FdSelect::Format3(item.to_owned_table()),
            ObjRefType::Format4(item) => FdSelect::Format4(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::postscript::FdSelect<'_>> for FdSelect {}

impl<'a> FontRead<'a> for FdSelect {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::postscript::FdSelect as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

impl From<FdSelectFormat0> for FdSelect {
    fn from(src: FdSelectFormat0) -> FdSelect {
        FdSelect::Format0(src)
    }
}

impl From<FdSelectFormat3> for FdSelect {
    fn from(src: FdSelectFormat3) -> FdSelect {
        FdSelect::Format3(src)
    }
}

impl From<FdSelectFormat4> for FdSelect {
    fn from(src: FdSelectFormat4) -> FdSelect {
        FdSelect::Format4(src)
    }
}

/// FdSelect format 0.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FdSelectFormat0 {
    /// FD selector array (one entry for each glyph).
    pub fds: Vec<u8>,
}

impl FdSelectFormat0 {
    /// Construct a new `FdSelectFormat0`
    pub fn new(fds: Vec<u8>) -> Self {
        Self {
            fds: fds.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for FdSelectFormat0 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (0 as u8).write_into(writer);
        self.fds.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FdSelectFormat0")
    }
}

impl Validate for FdSelectFormat0 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::postscript::FdSelectFormat0<'a>> for FdSelectFormat0 {
    fn from_obj_ref(
        obj: &read_fonts::tables::postscript::FdSelectFormat0<'a>,
        _: FontData,
    ) -> Self {
        let offset_data = obj.offset_data();
        FdSelectFormat0 {
            fds: obj.fds().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::postscript::FdSelectFormat0<'a>> for FdSelectFormat0 {}

impl<'a> FontRead<'a> for FdSelectFormat0 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::postscript::FdSelectFormat0 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// FdSelect format 3.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FdSelectFormat3 {
    /// Range3 array.
    pub ranges: Vec<FdSelectRange3>,
    /// Sentinel GID. Set equal to the number of glyphs in the font.
    pub sentinel: u16,
}

impl FdSelectFormat3 {
    /// Construct a new `FdSelectFormat3`
    pub fn new(ranges: Vec<FdSelectRange3>, sentinel: u16) -> Self {
        Self {
            ranges: ranges.into_iter().map(Into::into).collect(),
            sentinel,
        }
    }
}

impl FontWrite for FdSelectFormat3 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (3 as u8).write_into(writer);
        (array_len(&self.ranges).unwrap() as u16).write_into(writer);
        self.ranges.write_into(writer);
        self.sentinel.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FdSelectFormat3")
    }
}

impl Validate for FdSelectFormat3 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("FdSelectFormat3", |ctx| {
            ctx.in_field("ranges", |ctx| {
                if self.ranges.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.ranges.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::postscript::FdSelectFormat3<'a>> for FdSelectFormat3 {
    fn from_obj_ref(
        obj: &read_fonts::tables::postscript::FdSelectFormat3<'a>,
        _: FontData,
    ) -> Self {
        let offset_data = obj.offset_data();
        FdSelectFormat3 {
            ranges: obj.ranges().to_owned_obj(offset_data),
            sentinel: obj.sentinel(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::postscript::FdSelectFormat3<'a>> for FdSelectFormat3 {}

impl<'a> FontRead<'a> for FdSelectFormat3 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::postscript::FdSelectFormat3 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// Range struct for FdSelect format 3.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FdSelectRange3 {
    /// First glyph index in range.
    pub first: u16,
    /// FD index for all glyphs in range.
    pub fd: u8,
}

impl FdSelectRange3 {
    /// Construct a new `FdSelectRange3`
    pub fn new(first: u16, fd: u8) -> Self {
        Self { first, fd }
    }
}

impl FontWrite for FdSelectRange3 {
    fn write_into(&self, writer: &mut TableWriter) {
        self.first.write_into(writer);
        self.fd.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FdSelectRange3")
    }
}

impl Validate for FdSelectRange3 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::postscript::FdSelectRange3> for FdSelectRange3 {
    fn from_obj_ref(obj: &read_fonts::tables::postscript::FdSelectRange3, _: FontData) -> Self {
        FdSelectRange3 {
            first: obj.first(),
            fd: obj.fd(),
        }
    }
}

/// FdSelect format 4.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FdSelectFormat4 {
    /// Range4 array.
    pub ranges: Vec<FdSelectRange4>,
    /// Sentinel GID. Set equal to the number of glyphs in the font.
    pub sentinel: u32,
}

impl FdSelectFormat4 {
    /// Construct a new `FdSelectFormat4`
    pub fn new(ranges: Vec<FdSelectRange4>, sentinel: u32) -> Self {
        Self {
            ranges: ranges.into_iter().map(Into::into).collect(),
            sentinel,
        }
    }
}

impl FontWrite for FdSelectFormat4 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (4 as u8).write_into(writer);
        (array_len(&self.ranges).unwrap() as u32).write_into(writer);
        self.ranges.write_into(writer);
        self.sentinel.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FdSelectFormat4")
    }
}

impl Validate for FdSelectFormat4 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("FdSelectFormat4", |ctx| {
            ctx.in_field("ranges", |ctx| {
                if self.ranges.len() > (u32::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.ranges.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::postscript::FdSelectFormat4<'a>> for FdSelectFormat4 {
    fn from_obj_ref(
        obj: &read_fonts::tables::postscript::FdSelectFormat4<'a>,
        _: FontData,
    ) -> Self {
        let offset_data = obj.offset_data();
        FdSelectFormat4 {
            ranges: obj.ranges().to_owned_obj(offset_data),
            sentinel: obj.sentinel(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::postscript::FdSelectFormat4<'a>> for FdSelectFormat4 {}

impl<'a> FontRead<'a> for FdSelectFormat4 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::postscript::FdSelectFormat4 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// Range struct for FdSelect format 4.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FdSelectRange4 {
    /// First glyph index in range.
    pub first: u32,
    /// FD index for all glyphs in range.
    pub fd: u16,
}

impl FdSelectRange4 {
    /// Construct a new `FdSelectRange4`
    pub fn new(first: u32, fd: u16) -> Self {
        Self { first, fd }
    }
}

impl FontWrite for FdSelectRange4 {
    fn write_into(&self, writer: &mut TableWriter) {
        self.first.write_into(writer);
        self.fd.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FdSelectRange4")
    }
}

impl Validate for FdSelectRange4 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl FromObjRef<read_fonts::tables::postscript::FdSelectRange4> for FdSelectRange4 {
    fn from_obj_ref(obj: &read_fonts::tables::postscript::FdSelectRange4, _: FontData) -> Self {
        FdSelectRange4 {
            first: obj.first(),
            fd: obj.fd(),
        }
    }
}
