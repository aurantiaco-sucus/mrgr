use std::collections::{BTreeMap, HashMap};
use std::fs;
use once_cell::sync::Lazy;
use crate::TextStyle;

pub(crate) struct BitmapFont {
    columns: i32,
    glyphs_full_width: BTreeMap<char, i32>,
    glyphs_half_width: BTreeMap<char, i32>,
    pub bitmaps: BTreeMap<TextStyle, skia_safe::Image>,
}

fn load_bitmap_image(path_base: &str, variant: &str) -> skia_safe::Image {
    let data = fs::read(format!("{path_base}-{variant}.png")).unwrap();
    let data = skia_safe::Data::new_copy(&data);
    skia_safe::Image::from_encoded(data).unwrap()
}

pub(crate) struct BitmapFontLookupResult {
    pub pos: (i32, i32),
    pub is_full_width: bool,
}

impl BitmapFont {
    pub fn load(path_base: &str) -> Option<Self> {
        let glyphs_full_width = fs::read_to_string(format!("{path_base}-gf.txt")).ok()?
            .chars().collect::<Vec<_>>();
        let glyphs_half_width = fs::read_to_string(format!("{path_base}-gh.txt")).ok()?
            .chars().collect::<Vec<_>>();
        let hw_begin = glyphs_full_width.len();
        let glyphs_full_width = glyphs_full_width.into_iter()
            .enumerate()
            .map(|(i, c)| (c, i as i32))
            .collect::<BTreeMap<_, _>>();
        let glyphs_half_width = glyphs_half_width.into_iter()
            .enumerate()
            .map(|(i, c)| (c, i as i32 + hw_begin as i32))
            .collect::<BTreeMap<_, _>>();
        let width_small = 16;
        let bitmaps = [
            (TextStyle::SmallNormal, load_bitmap_image(path_base, "n")),
            (TextStyle::SmallBold, load_bitmap_image(path_base, "nb")),
            (TextStyle::SmallItalic, load_bitmap_image(path_base, "ni")),
            (TextStyle::SmallBoldItalic, load_bitmap_image(path_base, "nbi")),
            (TextStyle::LargeNormal, load_bitmap_image(path_base, "l")),
            (TextStyle::LargeBold, load_bitmap_image(path_base, "lb")),
            (TextStyle::LargeItalic, load_bitmap_image(path_base, "li")),
            (TextStyle::LargeBoldItalic, load_bitmap_image(path_base, "lbi")),
        ].iter().cloned().collect::<BTreeMap<_, _>>();
        let columns = bitmaps[&TextStyle::SmallNormal].width() / width_small;
        Some(Self {
            columns,
            glyphs_full_width,
            glyphs_half_width,
            bitmaps,
        })
    }
    
    pub fn lookup(&self, c: char) -> Option<BitmapFontLookupResult> {
        if let Some(i) = self.glyphs_full_width.get(&c) {
            let pos = (i % self.columns, i / self.columns);
            return Some(BitmapFontLookupResult {
                pos,
                is_full_width: true,
            });
        }
        if let Some(i) = self.glyphs_half_width.get(&c) {
            let pos = (i % self.columns, i / self.columns);
            return Some(BitmapFontLookupResult {
                pos,
                is_full_width: false,
            });
        }
        None
    }
    
    pub fn lookup_or_default(&self, c: char) -> BitmapFontLookupResult {
        self.lookup(c).unwrap_or(self.lookup('?').unwrap())
    }
}

static mut FONT: Option<BitmapFont> = None;

pub(crate) fn font() -> &'static BitmapFont {
    unsafe { FONT.as_ref().unwrap() }
}

pub fn is_font_loaded() -> bool {
    unsafe { FONT.is_some() }
}

pub fn load_bitmap_font(path_base: &str) {
    unsafe { FONT = BitmapFont::load(path_base); }
}

pub(crate) static mut PATH_IMAGE_POOL: Lazy<HashMap<String, skia_safe::Image>> = Lazy::new(|| HashMap::new());

pub fn load_image(path: &str) {
    let data = fs::read(path).unwrap();
    let data = skia_safe::Data::new_copy(&data);
    let image = skia_safe::Image::from_encoded(data).unwrap();
    unsafe { PATH_IMAGE_POOL.insert(path.to_string(), image); }
}

pub fn unload_image(path: &str) {
    unsafe { PATH_IMAGE_POOL.remove(path); }
}

pub(crate) fn image(path: &str) -> &'static skia_safe::Image {
    if let Some(image) = unsafe { PATH_IMAGE_POOL.get(path) } {
        image
    } else {
        load_image(path);
        unsafe { PATH_IMAGE_POOL.get(path).unwrap() }
    }
}