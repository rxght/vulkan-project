use cgmath::Vector2;

use super::{nom, tables, Contour, Glyph};

struct GlyphDescription {
    contour_count: i16,
    min_x: u16,
    min_y: u16,
    max_x: u16,
    max_y: u16,
}

pub fn load_glyphs(data: &[u8]) -> Option<Vec<Glyph>> {
    let tables = tables::get_tables(data)?;
    let head_table = tables::get_head_table(data)?;
    let location_table = tables.iter().find(|p| p.tag == "loca")?;
    let glyph_table_offset = tables.iter().find(|p| p.tag == "glyf")?.offset as usize;

    let mut idx = location_table.offset as usize;
    let end_idx = idx + location_table.length as usize;
    let dword_offsets = head_table.index_to_location_format == 1;

    let mut glyphs = Vec::new();

    if dword_offsets {
        let mut previous_offset = nom::<u32>(data, &mut idx)?;
        while idx < end_idx {
            let offset = nom::<u32>(data, &mut idx)?;
            if offset == previous_offset {
                glyphs.push(Glyph {
                    contours: Vec::with_capacity(0),
                });
            } else {
                let actual_offset = glyph_table_offset + previous_offset as usize;
                glyphs.push(parse_glyph(data, actual_offset)?);
            }
            previous_offset = offset;
        }
    } else {
        let mut previous_offset = nom::<u16>(data, &mut idx)?;
        while idx < end_idx {
            let offset = nom::<u16>(data, &mut idx)?;
            if offset == previous_offset {
                glyphs.push(Glyph {
                    contours: Vec::with_capacity(0),
                });
            } else {
                let actual_offset = glyph_table_offset + 2 * previous_offset as usize;
                glyphs.push(match parse_glyph(data, actual_offset) {
                    Some(v) => v,
                    None => {
                        println!("Problem! parse_glyph failed with offset = {actual_offset}");
                        Glyph {
                            contours: Vec::with_capacity(0),
                        }
                    }
                });
                println!("")
            }
            previous_offset = offset;
        }
    }

    return Some(glyphs);
}

fn parse_glyph(data: &[u8], mut offset: usize) -> Option<Glyph> {
    let idx = &mut offset;

    let glyph = GlyphDescription {
        contour_count: nom(data, idx)?,
        min_x: nom(data, idx)?,
        min_y: nom(data, idx)?,
        max_x: nom(data, idx)?,
        max_y: nom(data, idx)?,
    };

    if glyph.contour_count < 0 {
        return Some(Glyph {
            contours: Vec::with_capacity(0),
        });
    }

    let contour_count = glyph.contour_count as usize;

    let mut end_indices: Vec<u16> = Vec::with_capacity(contour_count);
    let instruction_count: u16;
    let mut flags: Vec<u8> = Vec::with_capacity(contour_count);
    let mut x_coordinates: Vec<i16> = Vec::with_capacity(contour_count);
    let mut y_coordinates: Vec<i16> = Vec::with_capacity(contour_count);

    let mut flag_repeat_counter: u8 = 0;
    let mut stored_flag: u8 = 0;

    for _ in 0..glyph.contour_count {
        end_indices.push(nom(data, idx)?);
    }
    instruction_count = nom(data, idx)?;

    *idx += instruction_count as usize;

    let mut flag_idx: usize = 0;
    for &end_idx in &end_indices
    // flags
    {
        while flag_idx <= end_idx as usize {
            if flag_repeat_counter > 0 {
                flags.push(stored_flag);
                flag_repeat_counter -= 1;
            } else {
                let flag: u8 = nom(data, idx)?;
                let repeat = flag & (1 << 3) != 0;
                if repeat {
                    stored_flag = flag;
                    flag_repeat_counter = nom(data, idx)?;
                    flags.push(flag);
                } else {
                    flags.push(flag);
                }
            }
            flag_idx += 1;
        }
    }
    for &flag in &flags
    // x-coordinates
    {
        let is_short = flag & (1 << 1) != 0;
        let coord;

        if is_short {
            let is_positive = flag & (1 << 4) != 0;
            let shawty = nom::<u8>(data, idx)? as i16;
            if is_positive {
                coord = shawty;
            } else {
                coord = -shawty;
            }
        } else {
            let repeat_previous = flag & (1 << 4) != 0;
            if repeat_previous {
                coord = 0;
            } else {
                coord = nom(data, idx)?;
            }
        }
        x_coordinates.push(coord);
    }
    for &flag in &flags
    // y-coordinates
    {
        let is_short = flag & (1 << 2) != 0;
        let coord;

        if is_short {
            let is_positive = flag & (1 << 5) != 0;
            let shawty = nom::<u8>(data, idx)? as i16;
            if is_positive {
                coord = shawty;
            } else {
                coord = -shawty;
            }
        } else {
            let repeat_previous = flag & (1 << 5) != 0;
            if repeat_previous {
                coord = 0;
            } else {
                coord = nom(data, idx)?;
            }
        }
        y_coordinates.push(coord);
    }

    let mut contours: Vec<Contour> = Vec::new();

    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut point_index = 0;
    for end_index in end_indices {
        let mut next_contour = Contour {
            points: Vec::new(),
            on_curve: Vec::new(),
        };
        while point_index as u16 <= end_index {
            x_pos += x_coordinates[point_index];
            y_pos += y_coordinates[point_index];
            let next_point = Vector2 {
                x: x_pos as f32,
                y: y_pos as f32,
            };

            let flag = flags[point_index];
            let on_curve = flag & 1 != 0;

            next_contour.points.push(next_point);
            next_contour.on_curve.push(on_curve);
            point_index += 1;
        }
        contours.push(next_contour);
    }

    return Some(Glyph { contours: contours });
}
