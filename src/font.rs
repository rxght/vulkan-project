use std::{
    collections::HashMap,
    io::Read,
    mem::{size_of, transmute},
    sync::{Arc, RwLock, Weak},
};

use cgmath::{num_traits::FromBytes, Vector2};

use self::{glyphs::load_glyphs, mapping::load_cmap};

pub mod outline;

mod glyphs;
mod mapping;
mod tables;

fn nom<T>(data: &[u8], idx: &mut usize) -> Option<T>
where
    T: FromBytes,
    <T as FromBytes>::Bytes: Sized,
{
    let value: T = T::from_be_bytes(unsafe { transmute(data.get(*idx)?) });
    *idx = *idx + size_of::<T>();
    return Some(value);
}

#[derive(Debug)]
pub struct Contour {
    pub points: Vec<Vector2<f32>>,
    pub on_curve: Vec<bool>,
}

pub struct Glyph {
    pub contours: Vec<Contour>,
}

struct FontInfo {
    scaling_factor: f32,
}

pub struct Font {
    font_info: FontInfo,
    character_map: HashMap<char, u16>,
    glyphs: Vec<Glyph>,
}

impl Font {
    fn new(path: &str) -> Option<Arc<Self>> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let glyphs = load_glyphs(&data)?;
        let character_map = load_cmap(&data)?;

        Some(Arc::new(Font {
            font_info: FontInfo {
                scaling_factor: 1.0,
            },
            character_map: character_map,
            glyphs: glyphs,
        }))
    }

    pub fn get_glyph(&self, c: char) -> &Glyph {
        let index = self.character_map.get(&c).cloned().unwrap_or(0);
        return match self.glyphs.get(index as usize) {
            Some(v) => v,
            None => &self.glyphs[0],
        };
    }
}

pub struct FontLoader {
    loaded_fonts: RwLock<HashMap<String, Weak<Font>>>,
}

impl FontLoader {
    pub fn new() -> Arc<FontLoader> {
        Arc::new(FontLoader {
            loaded_fonts: RwLock::new(HashMap::new()),
        })
    }

    pub fn load_ttf(&self, path: &str) -> Option<Arc<Font>> {
        match self.loaded_fonts.read() {
            Ok(guard) => match guard.get(path).and_then(Weak::upgrade) {
                Some(font) => Some(font),
                None => Font::new(path),
            },
            Err(_) => panic!("font loader poisoned"),
        }
    }

    fn cleanup_unused_fonts(&self) {
        if let Ok(mut guard) = self.loaded_fonts.write() {
            guard.retain(|_, v| v.strong_count() != 0);
        }
    }
}
