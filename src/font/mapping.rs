use std::collections::HashMap;

use super::{nom, tables};

pub fn load_cmap(data: &[u8]) -> Option<HashMap<char, u16>> {
    let cmap_table = tables::get_tables(data)?
        .into_iter()
        .find(|table| table.tag == "cmap")?;
    let mut idx = &mut (cmap_table.offset as usize);

    let _version: u16 = nom(data, &mut idx)?;
    let num_subtables: u16 = nom(data, &mut idx)?;

    let mut map_offset = None;
    for _ in 0..num_subtables {
        let platform_id: u16 = nom(data, &mut idx)?;
        let specific_id: u16 = nom(data, &mut idx)?;
        let offset: u32 = nom(data, &mut idx)?;

        if let (0, 0) | (0, 1) | (0, 3) = (platform_id, specific_id) {
            map_offset = Some(offset);
            break;
        }
    }

    let subtable_offset = match map_offset {
        Some(val) => val,
        None => {
            println!("Failed to find a valid cmap");
            return None;
        }
    };
    *idx = (cmap_table.offset + subtable_offset) as usize;

    let format: u16 = nom(data, &mut idx)?;
    match format {
        4 => {
            let subtable_length: u16 = nom(data, &mut idx)?;
            let _end_idx = subtable_offset + subtable_length as u32;

            let _language: u16 = nom(data, &mut idx)?;
            let seg_count: u16 = nom::<u16>(data, &mut idx)? / 2;
            let _search_range: u16 = nom(data, &mut idx)?;
            let _entry_selector: u16 = nom(data, &mut idx)?;
            let _range_shift: u16 = nom(data, &mut idx)?;
            let mut end_codes = Vec::<u16>::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                end_codes.push(nom(data, &mut idx)?);
            }
            let _reserved: u16 = nom(data, &mut idx)?;
            let mut start_codes = Vec::<u16>::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                start_codes.push(nom(data, &mut idx)?);
            }
            let mut deltas = Vec::<u16>::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                deltas.push(nom(data, &mut idx)?);
            }
            let range_offset_start = *idx;
            let mut range_offsets = Vec::<u16>::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                range_offsets.push(nom(data, &mut idx)?);
            }

            let mut character_map: HashMap<char, u16> = HashMap::new();

            for i in 0..seg_count as usize {
                if range_offsets[i] == 0 {
                    for code in start_codes[i]..end_codes[i] {
                        let code = code as u32;
                        let char_code = match char::from_u32(code) {
                            Some(v) => v,
                            None => continue,
                        };
                        let mapped_index = ((code + deltas[i] as u32) % 65536) as u16;
                        character_map.insert(char_code, mapped_index);
                    }
                } else {
                    let range_offset = range_offset_start + i * 2 + range_offsets[i] as usize;
                    *idx = range_offset;

                    for code in start_codes[i]..end_codes[i] {
                        let code = code as u32;
                        let char_code = match char::from_u32(code) {
                            Some(v) => v,
                            None => continue,
                        };
                        let mut mapped_index = nom::<u16>(data, &mut idx)?;
                        if mapped_index != 0 {
                            mapped_index = ((mapped_index as u32 + deltas[i] as u32) % 65536) as u16
                        }
                        character_map.insert(char_code, mapped_index);
                    }
                }
            }
            return Some(character_map);
        }
        12 => {
            todo!()
        }
        unimplemented_format => {
            println!("Unimplemented cmap format used: {unimplemented_format}");
            return None;
        }
    }
}
