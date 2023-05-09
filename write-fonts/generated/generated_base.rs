// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// The [BASE](https://learn.microsoft.com/en-us/typography/opentype/spec/base) (Baseline) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Base {
    /// Offset to horizontal Axis table, from beginning of BASE table (may be NULL)
    pub horiz_axis: NullableOffsetMarker<Axis>,
    /// Offset to vertical Axis table, from beginning of BASE table (may be NULL)
    pub vert_axis: NullableOffsetMarker<Axis>,
    /// Offset to Item Variation Store table, from beginning of BASE table (may be null)
    pub item_var_store: NullableOffsetMarker<ItemVariationStore, WIDTH_32>,
}

impl Base {
    /// Construct a new `Base`
    pub fn new(horiz_axis: Option<Axis>, vert_axis: Option<Axis>) -> Self {
        Self {
            horiz_axis: horiz_axis.into(),
            vert_axis: vert_axis.into(),
            ..Default::default()
        }
    }
}

impl FontWrite for Base {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.compute_version() as MajorMinor;
        version.write_into(writer);
        self.horiz_axis.write_into(writer);
        self.vert_axis.write_into(writer);
        version
            .compatible((1, 1))
            .then(|| self.item_var_store.write_into(writer));
    }
    fn table_type(&self) -> TableType {
        TableType::TopLevel(Base::TAG)
    }
}

impl Validate for Base {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Base", |ctx| {
            ctx.in_field("horiz_axis", |ctx| {
                self.horiz_axis.validate_impl(ctx);
            });
            ctx.in_field("vert_axis", |ctx| {
                self.vert_axis.validate_impl(ctx);
            });
            ctx.in_field("item_var_store", |ctx| {
                self.item_var_store.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Base {
    const TAG: Tag = Tag::new(b"BASE");
}

impl<'a> FromObjRef<read_fonts::tables::base::Base<'a>> for Base {
    fn from_obj_ref(obj: &read_fonts::tables::base::Base<'a>, _: FontData) -> Self {
        Base {
            horiz_axis: obj.horiz_axis().to_owned_table(),
            vert_axis: obj.vert_axis().to_owned_table(),
            item_var_store: obj.item_var_store().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::Base<'a>> for Base {}

impl<'a> FontRead<'a> for Base {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::Base as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [Axis Table](https://learn.microsoft.com/en-us/typography/opentype/spec/base#axis-tables-horizaxis-and-vertaxis)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Axis {
    /// Offset to BaseTagList table, from beginning of Axis table (may
    /// be NULL)
    pub base_tag_list: NullableOffsetMarker<BaseTagList>,
    /// Offset to BaseScriptList table, from beginning of Axis table
    pub base_script_list: OffsetMarker<BaseScriptList>,
}

impl Axis {
    /// Construct a new `Axis`
    pub fn new(base_tag_list: Option<BaseTagList>, base_script_list: BaseScriptList) -> Self {
        Self {
            base_tag_list: base_tag_list.into(),
            base_script_list: base_script_list.into(),
        }
    }
}

impl FontWrite for Axis {
    fn write_into(&self, writer: &mut TableWriter) {
        self.base_tag_list.write_into(writer);
        self.base_script_list.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("Axis")
    }
}

impl Validate for Axis {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Axis", |ctx| {
            ctx.in_field("base_tag_list", |ctx| {
                self.base_tag_list.validate_impl(ctx);
            });
            ctx.in_field("base_script_list", |ctx| {
                self.base_script_list.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::Axis<'a>> for Axis {
    fn from_obj_ref(obj: &read_fonts::tables::base::Axis<'a>, _: FontData) -> Self {
        Axis {
            base_tag_list: obj.base_tag_list().to_owned_table(),
            base_script_list: obj.base_script_list().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::Axis<'a>> for Axis {}

impl<'a> FontRead<'a> for Axis {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::Axis as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [BaseTagList Table](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basetaglist-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseTagList {
    /// Array of 4-byte baseline identification tags — must be in
    /// alphabetical order
    pub baseline_tags: Vec<Tag>,
}

impl BaseTagList {
    /// Construct a new `BaseTagList`
    pub fn new(baseline_tags: Vec<Tag>) -> Self {
        Self {
            baseline_tags: baseline_tags.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for BaseTagList {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.baseline_tags).unwrap() as u16).write_into(writer);
        self.baseline_tags.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseTagList")
    }
}

impl Validate for BaseTagList {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseTagList", |ctx| {
            ctx.in_field("baseline_tags", |ctx| {
                if self.baseline_tags.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseTagList<'a>> for BaseTagList {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseTagList<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        BaseTagList {
            baseline_tags: obj.baseline_tags().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseTagList<'a>> for BaseTagList {}

impl<'a> FontRead<'a> for BaseTagList {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseTagList as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [BaseScriptList Table](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basescriptlist-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseScriptList {
    /// Array of BaseScriptRecords, in alphabetical order by
    /// baseScriptTag
    pub base_script_records: Vec<BaseScriptRecord>,
}

impl BaseScriptList {
    /// Construct a new `BaseScriptList`
    pub fn new(base_script_records: Vec<BaseScriptRecord>) -> Self {
        Self {
            base_script_records: base_script_records.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for BaseScriptList {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.base_script_records).unwrap() as u16).write_into(writer);
        self.base_script_records.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseScriptList")
    }
}

impl Validate for BaseScriptList {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseScriptList", |ctx| {
            ctx.in_field("base_script_records", |ctx| {
                if self.base_script_records.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.base_script_records.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseScriptList<'a>> for BaseScriptList {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseScriptList<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        BaseScriptList {
            base_script_records: obj.base_script_records().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseScriptList<'a>> for BaseScriptList {}

impl<'a> FontRead<'a> for BaseScriptList {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseScriptList as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [BaseScriptRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basescriptrecord)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseScriptRecord {
    /// 4-byte script identification tag
    pub base_script_tag: Tag,
    /// Offset to BaseScript table, from beginning of BaseScriptList
    pub base_script: OffsetMarker<BaseScript>,
}

impl BaseScriptRecord {
    /// Construct a new `BaseScriptRecord`
    pub fn new(base_script_tag: Tag, base_script: BaseScript) -> Self {
        Self {
            base_script_tag,
            base_script: base_script.into(),
        }
    }
}

impl FontWrite for BaseScriptRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.base_script_tag.write_into(writer);
        self.base_script.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseScriptRecord")
    }
}

impl Validate for BaseScriptRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseScriptRecord", |ctx| {
            ctx.in_field("base_script", |ctx| {
                self.base_script.validate_impl(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::tables::base::BaseScriptRecord> for BaseScriptRecord {
    fn from_obj_ref(
        obj: &read_fonts::tables::base::BaseScriptRecord,
        offset_data: FontData,
    ) -> Self {
        BaseScriptRecord {
            base_script_tag: obj.base_script_tag(),
            base_script: obj.base_script(offset_data).to_owned_table(),
        }
    }
}

/// [BaseScript Table](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basescript-table)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseScript {
    /// Offset to BaseValues table, from beginning of BaseScript table (may be NULL)
    pub base_values: NullableOffsetMarker<BaseValues>,
    /// Offset to MinMax table, from beginning of BaseScript table (may be NULL)
    pub default_min_max: NullableOffsetMarker<MinMax>,
    /// Array of BaseLangSysRecords, in alphabetical order by
    /// BaseLangSysTag
    pub base_lang_sys_records: Vec<BaseLangSysRecord>,
}

impl BaseScript {
    /// Construct a new `BaseScript`
    pub fn new(
        base_values: Option<BaseValues>,
        default_min_max: Option<MinMax>,
        base_lang_sys_records: Vec<BaseLangSysRecord>,
    ) -> Self {
        Self {
            base_values: base_values.into(),
            default_min_max: default_min_max.into(),
            base_lang_sys_records: base_lang_sys_records.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for BaseScript {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.base_values.write_into(writer);
        self.default_min_max.write_into(writer);
        (array_len(&self.base_lang_sys_records).unwrap() as u16).write_into(writer);
        self.base_lang_sys_records.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseScript")
    }
}

impl Validate for BaseScript {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseScript", |ctx| {
            ctx.in_field("base_values", |ctx| {
                self.base_values.validate_impl(ctx);
            });
            ctx.in_field("default_min_max", |ctx| {
                self.default_min_max.validate_impl(ctx);
            });
            ctx.in_field("base_lang_sys_records", |ctx| {
                if self.base_lang_sys_records.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.base_lang_sys_records.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseScript<'a>> for BaseScript {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseScript<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        BaseScript {
            base_values: obj.base_values().to_owned_table(),
            default_min_max: obj.default_min_max().to_owned_table(),
            base_lang_sys_records: obj.base_lang_sys_records().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseScript<'a>> for BaseScript {}

impl<'a> FontRead<'a> for BaseScript {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseScript as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [BaseLangSysRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/base#baselangsysrecord)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseLangSysRecord {
    /// 4-byte language system identification tag
    pub base_lang_sys_tag: Tag,
    /// Offset to MinMax table, from beginning of BaseScript table
    pub min_max: OffsetMarker<MinMax>,
}

impl BaseLangSysRecord {
    /// Construct a new `BaseLangSysRecord`
    pub fn new(base_lang_sys_tag: Tag, min_max: MinMax) -> Self {
        Self {
            base_lang_sys_tag,
            min_max: min_max.into(),
        }
    }
}

impl FontWrite for BaseLangSysRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.base_lang_sys_tag.write_into(writer);
        self.min_max.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseLangSysRecord")
    }
}

impl Validate for BaseLangSysRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseLangSysRecord", |ctx| {
            ctx.in_field("min_max", |ctx| {
                self.min_max.validate_impl(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::tables::base::BaseLangSysRecord> for BaseLangSysRecord {
    fn from_obj_ref(
        obj: &read_fonts::tables::base::BaseLangSysRecord,
        offset_data: FontData,
    ) -> Self {
        BaseLangSysRecord {
            base_lang_sys_tag: obj.base_lang_sys_tag(),
            min_max: obj.min_max(offset_data).to_owned_table(),
        }
    }
}

/// [BaseValues](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basevalues-table) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseValues {
    /// Index number of default baseline for this script — equals
    /// index position of baseline tag in baselineTags array of the
    /// BaseTagList
    pub default_baseline_index: u16,
    /// Array of offsets to BaseCoord tables, from beginning of
    /// BaseValues table — order matches baselineTags array in the
    /// BaseTagList
    pub base_coords: Vec<OffsetMarker<BaseCoord>>,
}

impl BaseValues {
    /// Construct a new `BaseValues`
    pub fn new(default_baseline_index: u16, base_coords: Vec<BaseCoord>) -> Self {
        Self {
            default_baseline_index,
            base_coords: base_coords.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for BaseValues {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.default_baseline_index.write_into(writer);
        (array_len(&self.base_coords).unwrap() as u16).write_into(writer);
        self.base_coords.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseValues")
    }
}

impl Validate for BaseValues {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseValues", |ctx| {
            ctx.in_field("base_coords", |ctx| {
                if self.base_coords.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.base_coords.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseValues<'a>> for BaseValues {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseValues<'a>, _: FontData) -> Self {
        BaseValues {
            default_baseline_index: obj.default_baseline_index(),
            base_coords: obj.base_coords().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseValues<'a>> for BaseValues {}

impl<'a> FontRead<'a> for BaseValues {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseValues as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [MinMax](https://learn.microsoft.com/en-us/typography/opentype/spec/base#minmax-table) table
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MinMax {
    /// Offset to BaseCoord table that defines the minimum extent
    /// value, from the beginning of MinMax table (may be NULL)
    pub min_coord: NullableOffsetMarker<BaseCoord>,
    /// Offset to BaseCoord table that defines maximum extent value,
    /// from the beginning of MinMax table (may be NULL)
    pub max_coord: NullableOffsetMarker<BaseCoord>,
    /// Array of FeatMinMaxRecords, in alphabetical order by
    /// featureTableTag
    pub feat_min_max_records: Vec<FeatMinMaxRecord>,
}

impl MinMax {
    /// Construct a new `MinMax`
    pub fn new(
        min_coord: Option<BaseCoord>,
        max_coord: Option<BaseCoord>,
        feat_min_max_records: Vec<FeatMinMaxRecord>,
    ) -> Self {
        Self {
            min_coord: min_coord.into(),
            max_coord: max_coord.into(),
            feat_min_max_records: feat_min_max_records.into_iter().map(Into::into).collect(),
        }
    }
}

impl FontWrite for MinMax {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        self.min_coord.write_into(writer);
        self.max_coord.write_into(writer);
        (array_len(&self.feat_min_max_records).unwrap() as u16).write_into(writer);
        self.feat_min_max_records.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("MinMax")
    }
}

impl Validate for MinMax {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MinMax", |ctx| {
            ctx.in_field("min_coord", |ctx| {
                self.min_coord.validate_impl(ctx);
            });
            ctx.in_field("max_coord", |ctx| {
                self.max_coord.validate_impl(ctx);
            });
            ctx.in_field("feat_min_max_records", |ctx| {
                if self.feat_min_max_records.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.feat_min_max_records.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::MinMax<'a>> for MinMax {
    fn from_obj_ref(obj: &read_fonts::tables::base::MinMax<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        MinMax {
            min_coord: obj.min_coord().to_owned_table(),
            max_coord: obj.max_coord().to_owned_table(),
            feat_min_max_records: obj.feat_min_max_records().to_owned_obj(offset_data),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::MinMax<'a>> for MinMax {}

impl<'a> FontRead<'a> for MinMax {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::MinMax as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

/// [FeatMinMaxRecord](https://learn.microsoft.com/en-us/typography/opentype/spec/base#baselangsysrecord)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatMinMaxRecord {
    /// 4-byte feature identification tag — must match feature tag in
    /// FeatureList
    pub feature_table_tag: Tag,
    /// Offset to BaseCoord table that defines the minimum extent
    /// value, from beginning of MinMax table (may be NULL)
    pub min_coord: NullableOffsetMarker<MinMax>,
    /// Offset to BaseCoord table that defines the maximum extent
    /// value, from beginning of MinMax table (may be NULL)
    pub max_coord: NullableOffsetMarker<MinMax>,
}

impl FeatMinMaxRecord {
    /// Construct a new `FeatMinMaxRecord`
    pub fn new(
        feature_table_tag: Tag,
        min_coord: Option<MinMax>,
        max_coord: Option<MinMax>,
    ) -> Self {
        Self {
            feature_table_tag,
            min_coord: min_coord.into(),
            max_coord: max_coord.into(),
        }
    }
}

impl FontWrite for FeatMinMaxRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.feature_table_tag.write_into(writer);
        self.min_coord.write_into(writer);
        self.max_coord.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("FeatMinMaxRecord")
    }
}

impl Validate for FeatMinMaxRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("FeatMinMaxRecord", |ctx| {
            ctx.in_field("min_coord", |ctx| {
                self.min_coord.validate_impl(ctx);
            });
            ctx.in_field("max_coord", |ctx| {
                self.max_coord.validate_impl(ctx);
            });
        })
    }
}

impl FromObjRef<read_fonts::tables::base::FeatMinMaxRecord> for FeatMinMaxRecord {
    fn from_obj_ref(
        obj: &read_fonts::tables::base::FeatMinMaxRecord,
        offset_data: FontData,
    ) -> Self {
        FeatMinMaxRecord {
            feature_table_tag: obj.feature_table_tag(),
            min_coord: obj.min_coord(offset_data).to_owned_table(),
            max_coord: obj.max_coord(offset_data).to_owned_table(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaseCoord {
    Format1(BaseCoordFormat1),
    Format2(BaseCoordFormat2),
    Format3(BaseCoordFormat3),
}

impl BaseCoord {
    /// Construct a new `BaseCoordFormat1` subtable
    pub fn format_1(coordinate: i16) -> Self {
        Self::Format1(BaseCoordFormat1::new(coordinate))
    }

    /// Construct a new `BaseCoordFormat2` subtable
    pub fn format_2(coordinate: i16, reference_glyph: u16, base_coord_point: u16) -> Self {
        Self::Format2(BaseCoordFormat2::new(
            coordinate,
            reference_glyph,
            base_coord_point,
        ))
    }

    /// Construct a new `BaseCoordFormat3` subtable
    pub fn format_3(coordinate: i16, device: Option<DeviceOrVariationIndex>) -> Self {
        Self::Format3(BaseCoordFormat3::new(coordinate, device))
    }
}

impl Default for BaseCoord {
    fn default() -> Self {
        Self::Format1(Default::default())
    }
}

impl FontWrite for BaseCoord {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
        }
    }
    fn table_type(&self) -> TableType {
        match self {
            Self::Format1(item) => item.table_type(),
            Self::Format2(item) => item.table_type(),
            Self::Format3(item) => item.table_type(),
        }
    }
}

impl Validate for BaseCoord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
            Self::Format3(item) => item.validate_impl(ctx),
        }
    }
}

impl FromObjRef<read_fonts::tables::base::BaseCoord<'_>> for BaseCoord {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseCoord, _: FontData) -> Self {
        use read_fonts::tables::base::BaseCoord as ObjRefType;
        match obj {
            ObjRefType::Format1(item) => BaseCoord::Format1(item.to_owned_table()),
            ObjRefType::Format2(item) => BaseCoord::Format2(item.to_owned_table()),
            ObjRefType::Format3(item) => BaseCoord::Format3(item.to_owned_table()),
        }
    }
}

impl FromTableRef<read_fonts::tables::base::BaseCoord<'_>> for BaseCoord {}

impl<'a> FontRead<'a> for BaseCoord {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseCoord as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}

impl From<BaseCoordFormat1> for BaseCoord {
    fn from(src: BaseCoordFormat1) -> BaseCoord {
        BaseCoord::Format1(src)
    }
}

impl From<BaseCoordFormat2> for BaseCoord {
    fn from(src: BaseCoordFormat2) -> BaseCoord {
        BaseCoord::Format2(src)
    }
}

impl From<BaseCoordFormat3> for BaseCoord {
    fn from(src: BaseCoordFormat3) -> BaseCoord {
        BaseCoord::Format3(src)
    }
}

/// [BaseCoordFormat1](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basecoord-format-1)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseCoordFormat1 {
    /// X or Y value, in design units
    pub coordinate: i16,
}

impl BaseCoordFormat1 {
    /// Construct a new `BaseCoordFormat1`
    pub fn new(coordinate: i16) -> Self {
        Self { coordinate }
    }
}

impl FontWrite for BaseCoordFormat1 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coordinate.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseCoordFormat1")
    }
}

impl Validate for BaseCoordFormat1 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseCoordFormat1<'a>> for BaseCoordFormat1 {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseCoordFormat1<'a>, _: FontData) -> Self {
        BaseCoordFormat1 {
            coordinate: obj.coordinate(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseCoordFormat1<'a>> for BaseCoordFormat1 {}

impl<'a> FontRead<'a> for BaseCoordFormat1 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseCoordFormat1 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [BaseCoordFormat2](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basecoord-format-2)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseCoordFormat2 {
    /// X or Y value, in design units
    pub coordinate: i16,
    /// Glyph ID of control glyph
    pub reference_glyph: u16,
    /// Index of contour point on the reference glyph
    pub base_coord_point: u16,
}

impl BaseCoordFormat2 {
    /// Construct a new `BaseCoordFormat2`
    pub fn new(coordinate: i16, reference_glyph: u16, base_coord_point: u16) -> Self {
        Self {
            coordinate,
            reference_glyph,
            base_coord_point,
        }
    }
}

impl FontWrite for BaseCoordFormat2 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.coordinate.write_into(writer);
        self.reference_glyph.write_into(writer);
        self.base_coord_point.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseCoordFormat2")
    }
}

impl Validate for BaseCoordFormat2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseCoordFormat2<'a>> for BaseCoordFormat2 {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseCoordFormat2<'a>, _: FontData) -> Self {
        BaseCoordFormat2 {
            coordinate: obj.coordinate(),
            reference_glyph: obj.reference_glyph(),
            base_coord_point: obj.base_coord_point(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseCoordFormat2<'a>> for BaseCoordFormat2 {}

impl<'a> FontRead<'a> for BaseCoordFormat2 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseCoordFormat2 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}

/// [BaseCoordFormat3](https://learn.microsoft.com/en-us/typography/opentype/spec/base#basecoord-format-3)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BaseCoordFormat3 {
    /// X or Y value, in design units
    pub coordinate: i16,
    /// Offset to Device table (non-variable font) / Variation Index
    /// table (variable font) for X or Y value, from beginning of
    /// BaseCoord table (may be NULL).
    pub device: NullableOffsetMarker<DeviceOrVariationIndex>,
}

impl BaseCoordFormat3 {
    /// Construct a new `BaseCoordFormat3`
    pub fn new(coordinate: i16, device: Option<DeviceOrVariationIndex>) -> Self {
        Self {
            coordinate,
            device: device.into(),
        }
    }
}

impl FontWrite for BaseCoordFormat3 {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (3 as u16).write_into(writer);
        self.coordinate.write_into(writer);
        self.device.write_into(writer);
    }
    fn table_type(&self) -> TableType {
        TableType::Named("BaseCoordFormat3")
    }
}

impl Validate for BaseCoordFormat3 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseCoordFormat3", |ctx| {
            ctx.in_field("device", |ctx| {
                self.device.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::base::BaseCoordFormat3<'a>> for BaseCoordFormat3 {
    fn from_obj_ref(obj: &read_fonts::tables::base::BaseCoordFormat3<'a>, _: FontData) -> Self {
        BaseCoordFormat3 {
            coordinate: obj.coordinate(),
            device: obj.device().to_owned_table(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::base::BaseCoordFormat3<'a>> for BaseCoordFormat3 {}

impl<'a> FontRead<'a> for BaseCoordFormat3 {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::base::BaseCoordFormat3 as FontRead>::read(data)
            .map(|x| x.to_owned_table())
    }
}
