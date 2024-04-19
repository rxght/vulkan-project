use crate::font::nom;

struct OffsetSubtable {
    scaler_type: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

pub struct TableDescriptor {
    pub tag: String,
    pub offset: u32,
    pub length: u32,
}

pub struct HeadTable {
    pub version: u32,
    pub revision: u32,
    pub checksum_adjustment: u32,
    pub magic_number: u32,
    pub flags: u16,
    pub units_per_em: u16,
    pub created: u32,
    pub modified: u32,
    pub x_min: u16,
    pub y_min: u16,
    pub x_max: u16,
    pub y_max: u16,
    pub mac_style: u16,
    pub lowest_pixels_per_em: u16,
    pub font_direction_hint: i16,
    pub index_to_location_format: i16,
    pub glyph_data_format: i16,
}

pub fn get_head_table(data: &[u8]) -> Option<HeadTable> {
    let tables = get_tables(data)?;
    let head_table_desc = tables.iter().find(|p| p.tag == "head")?;

    let idx = &mut (head_table_desc.offset as usize);

    Some(HeadTable {
        version: nom(data, idx)?,
        revision: nom(data, idx)?,
        checksum_adjustment: nom(data, idx)?,
        magic_number: nom(data, idx)?,
        flags: nom(data, idx)?,
        units_per_em: nom(data, idx)?,
        created: nom(data, idx)?,
        modified: nom(data, idx)?,
        x_min: nom(data, idx)?,
        y_min: nom(data, idx)?,
        x_max: nom(data, idx)?,
        y_max: nom(data, idx)?,
        mac_style: nom(data, idx)?,
        lowest_pixels_per_em: nom(data, idx)?,
        font_direction_hint: nom(data, idx)?,
        index_to_location_format: nom(data, idx)?,
        glyph_data_format: nom(data, idx)?,
    })
}

pub fn get_tables(data: &[u8]) -> Option<Vec<TableDescriptor>> {
    let mut idx = 0;

    let offset_subtable = OffsetSubtable {
        scaler_type: nom(&data, &mut idx)?,
        num_tables: nom(&data, &mut idx)?,
        search_range: nom(&data, &mut idx)?,
        entry_selector: nom(&data, &mut idx)?,
        range_shift: nom(&data, &mut idx)?,
    };

    let mut tables = Vec::new();
    for _ in 0..offset_subtable.num_tables {
        let tag: u32 = nom(&data, &mut idx)?;
        let _checksum: u32 = nom(&data, &mut idx)?;
        let offset: u32 = nom(&data, &mut idx)?;
        let length: u32 = nom(&data, &mut idx)?;

        let tag_string = unsafe {
            let tag_bytes = tag.to_be_bytes();
            std::str::from_utf8_unchecked(&tag_bytes)
                .trim_end()
                .to_string()
        };
        let entry = TableDescriptor {
            tag: tag_string,
            offset: offset,
            length: length,
        };
        tables.push(entry);
    }
    return Some(tables);
}
