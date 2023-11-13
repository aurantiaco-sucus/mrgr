
pub struct Canvas<'a> {
    pub(crate) skia_canvas: &'a mut skia_safe::Canvas
}

impl Canvas {
    pub fn skia(&self) -> &mut skia_safe::Canvas {
        self.skia_canvas
    }

    pub fn image_path(&self, path: &str, pos: (i32, i32), size: impl Into<Option<(i32, i32)>>) {

    }

    pub fn image_key(&self, key: usize, pos: (i32, i32), size: impl Into<Option<(i32, i32)>>) {

    }

    pub fn text_normal(&self, text: &str, pos: (i32, i32), size: (i32, i32)) {

    }

    pub fn text_big(&self, text: &str, pos: (i32, i32), size: (i32, i32)) {

    }

    pub fn
}