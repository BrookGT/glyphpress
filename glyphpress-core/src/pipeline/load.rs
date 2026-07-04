//! Load SFNT font and parse core tables.


use crate::error::{GlyphError, GlyphResult};
use crate::sfnt::directory::TableDirectory;
use crate::tables::{
    cmap::CmapTable, glyf::GlyfTable, head::HeadTable, hhea::HheaTable, hmtx::HmtxTable,
    loca::LocaTable, maxp::MaxpTable, name::NameTable, os2::Os2Table, post::PostTable,
};

pub struct LoadedFont<'a> {
    pub data: &'a [u8],
    pub directory: TableDirectory,
    pub head: HeadTable,
    pub hhea: HheaTable,
    pub maxp: MaxpTable,
    pub loca: LocaTable,
    pub hmtx: HmtxTable,
    pub cmap: CmapTable,
    pub name: NameTable,
    pub os2: Option<Os2Table>,
    pub post: PostTable,
    pub glyf: GlyfTable<'a>,
}

impl<'a> LoadedFont<'a> {
    pub fn open(data: &'a [u8]) -> GlyphResult<Self> {
        let directory = TableDirectory::parse(data)?;
        let head = HeadTable::parse(directory.table_bytes(data, 0x68656164)?)?;
        let hhea = HheaTable::parse(directory.table_bytes(data, 0x68686561)?)?;
        let maxp = MaxpTable::parse(directory.table_bytes(data, 0x6D617870)?)?;
        let loca = LocaTable::parse(
            directory.table_bytes(data, 0x6C6F6361)?,
            head.uses_long_loca(),
            maxp.num_glyphs,
        )?;
        let hmtx = HmtxTable::parse(
            directory.table_bytes(data, 0x686D7478)?,
            hhea.number_of_h_metrics,
            maxp.num_glyphs,
        )?;
        let cmap = CmapTable::parse(directory.table_bytes(data, 0x636D6170)?)?;
        let name = NameTable::parse(directory.table_bytes(data, 0x6E616D65)?)?;
        let os2 = directory
            .table_bytes(data, 0x4F532F32)
            .ok()
            .map(Os2Table::parse)
            .transpose()?;
        let post = PostTable::parse(
            directory.table_bytes(data, 0x706F7374)?,
            maxp.num_glyphs,
        )?;
        let glyf_bytes = directory.table_bytes(data, 0x676C7966)?;
        let glyf = GlyfTable::new(glyf_bytes);
        Ok(Self {
            data,
            directory,
            head,
            hhea,
            maxp,
            loca,
            hmtx,
            cmap,
            name,
            os2,
            post,
            glyf,
        })
    }

    pub fn glyph_bytes(&self, gid: u16) -> GlyphResult<&'a [u8]> {
        let (start, end) = self.loca.glyph_range(gid)?;
        self.glyf.slice_for_range(start, end)
    }
}

impl<'a> LoadedFont<'a> {
    pub fn num_glyphs(&self) -> u16 { self.maxp.num_glyphs }
    pub fn family(&self) -> crate::GlyphResult<Option<String>> { self.name.family_name() }
    pub fn verify_core_checksums(&self) -> crate::GlyphResult<()> {
        self.directory.verify_checksum(self.data, 0x68656164)?;
        Ok(())
    }
}

/* volume */

impl<'a> LoadedFont<'a> {
    pub fn table_tags(&self) -> impl Iterator<Item = u32> + '_ {
        self.directory.tags()
    }
}
