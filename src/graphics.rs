use std::sync::MutexGuard;
use log::info;
use crate::is_font_loaded;
use crate::runtime::runtime;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextStyle {
    SmallNormal,
    SmallBold,
    SmallItalic,
    SmallBoldItalic,
    LargeNormal,
    LargeBold,
    LargeItalic,
    LargeBoldItalic,
}

impl TextStyle {
    pub fn is_small(&self) -> bool {
        matches!(self,
            Self::SmallNormal |
            Self::SmallBold |
            Self::SmallItalic |
            Self::SmallBoldItalic)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NinePatch {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) right: i32,
    pub(crate) bottom: i32,
}

impl NinePatch {
    pub fn from_all(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from_uni(border: i32) -> Self {
        Self {
            left: border,
            top: border,
            right: border,
            bottom: border,
        }
    }

    pub fn from_hv(horizontal: i32, vertical: i32) -> Self {
        Self {
            left: horizontal,
            top: vertical,
            right: horizontal,
            bottom: vertical,
        }
    }
}

pub struct Canvas {
    pub(crate) surface: MutexGuard<'static, skia_safe::Surface>
}

pub fn paint<F: FnOnce(Canvas)>(func: F) {
    let canvas = Canvas {
        surface: runtime().surface.lock().unwrap()
    };
    func(canvas);
}

impl Canvas {
    pub fn image(&mut self, path: &str, pos: (i32, i32), size: impl Into<Option<(i32, i32)>>) {
        let canvas = self.surface.canvas();
        let image = crate::image(path);
        let size = size.into().unwrap_or((image.width(), image.height()));
        let src = skia_safe::Rect::from_xywh(0.0, 0.0, image.width() as f32, image.height() as f32);
        let dst = skia_safe::Rect::from_xywh(pos.0 as f32, pos.1 as f32, size.0 as f32, size.1 as f32);
        canvas.draw_image_rect(
            image,
            Some((&src, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst,
            &skia_safe::Paint::default()
        );
    }

    pub fn text(&mut self, text: &str, style: TextStyle, pos: (i32, i32), line_width: i32) {
        let canvas = self.surface.canvas();
        if !is_font_loaded() { return; }
        let font = crate::font();
        let full_width = if style.is_small() { 16 } else { 32 };

        let mut x = pos.0;
        let mut y = pos.1;
        for ch in text.chars() {
            if ch == '\r' { continue; }
            if ch == '\n' {
                x = pos.0;
                y += full_width;
                continue;
            }
            let result = font.lookup_or_default(ch);
            let width = if result.is_full_width { full_width } else { full_width / 2 };
            if x + width > pos.0 + line_width {
                x = pos.0;
                y += full_width;
            }
            let bitmap = &font.bitmaps[&style];
            let src = skia_safe::Rect::from_xywh(
                (result.pos.0 * full_width) as f32,
                (result.pos.1 * full_width) as f32,
                width as f32,
                full_width as f32
            );
            let dst = skia_safe::Rect::from_xywh(
                x as f32,
                y as f32,
                width as f32,
                full_width as f32
            );
            x += width;
            canvas.draw_image_rect(
                bitmap,
                Some((&src, skia_safe::canvas::SrcRectConstraint::Fast)),
                dst,
                &skia_safe::Paint::default()
            );
        }
    }

    pub fn nine_patch(&mut self, path: &str, conf: NinePatch, pos: (i32, i32), size: (i32, i32)) {
        let canvas = self.surface.canvas();
        let image = crate::image(path);
        if image.width() < conf.left + conf.right || image.height() < conf.top + conf.bottom {
            info!("Nine patch image is too small!");
            return;
        }
        let src_tl = skia_safe::Rect::from_xywh(0.0, 0.0, conf.left as f32, conf.top as f32);
        let src_t = skia_safe::Rect::from_xywh(conf.left as f32, 0.0, (image.width() - conf.left - conf.right) as f32, conf.top as f32);
        let src_tr = skia_safe::Rect::from_xywh((image.width() - conf.right) as f32, 0.0, conf.right as f32, conf.top as f32);
        let src_l = skia_safe::Rect::from_xywh(0.0, conf.top as f32, conf.left as f32, (image.height() - conf.top - conf.bottom) as f32);
        let src_c = skia_safe::Rect::from_xywh(conf.left as f32, conf.top as f32, (image.width() - conf.left - conf.right) as f32, (image.height() - conf.top - conf.bottom) as f32);
        let src_r = skia_safe::Rect::from_xywh((image.width() - conf.right) as f32, conf.top as f32, conf.right as f32, (image.height() - conf.top - conf.bottom) as f32);
        let src_bl = skia_safe::Rect::from_xywh(0.0, (image.height() - conf.bottom) as f32, conf.left as f32, conf.bottom as f32);
        let src_b = skia_safe::Rect::from_xywh(conf.left as f32, (image.height() - conf.bottom) as f32, (image.width() - conf.left - conf.right) as f32, conf.bottom as f32);
        let src_br = skia_safe::Rect::from_xywh((image.width() - conf.right) as f32, (image.height() - conf.bottom) as f32, conf.right as f32, conf.bottom as f32);
        let dst_tl = skia_safe::Rect::from_xywh(pos.0 as f32, pos.1 as f32, conf.left as f32, conf.top as f32);
        let dst_t = skia_safe::Rect::from_xywh((pos.0 + conf.left) as f32, pos.1 as f32, (size.0 - conf.left - conf.right) as f32, conf.top as f32);
        let dst_tr = skia_safe::Rect::from_xywh((pos.0 + size.0 - conf.right) as f32, pos.1 as f32, conf.right as f32, conf.top as f32);
        let dst_l = skia_safe::Rect::from_xywh(pos.0 as f32, (pos.1 + conf.top) as f32, conf.left as f32, (size.1 - conf.top - conf.bottom) as f32);
        let dst_c = skia_safe::Rect::from_xywh((pos.0 + conf.left) as f32, (pos.1 + conf.top) as f32, (size.0 - conf.left - conf.right) as f32, (size.1 - conf.top - conf.bottom) as f32);
        let dst_r = skia_safe::Rect::from_xywh((pos.0 + size.0 - conf.right) as f32, (pos.1 + conf.top) as f32, conf.right as f32, (size.1 - conf.top - conf.bottom) as f32);
        let dst_bl = skia_safe::Rect::from_xywh(pos.0 as f32, (pos.1 + size.1 - conf.bottom) as f32, conf.left as f32, conf.bottom as f32);
        let dst_b = skia_safe::Rect::from_xywh((pos.0 + conf.left) as f32, (pos.1 + size.1 - conf.bottom) as f32, (size.0 - conf.left - conf.right) as f32, conf.bottom as f32);
        let dst_br = skia_safe::Rect::from_xywh((pos.0 + size.0 - conf.right) as f32, (pos.1 + size.1 - conf.bottom) as f32, conf.right as f32, conf.bottom as f32);
        canvas.draw_image_rect(
            image,
            Some((&src_tl, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_tl,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_t, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_t,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_tr, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_tr,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_l, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_l,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_c, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_c,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_r, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_r,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_bl, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_bl,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_b, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_b,
            &skia_safe::Paint::default()
        );
        canvas.draw_image_rect(
            image,
            Some((&src_br, skia_safe::canvas::SrcRectConstraint::Fast)),
            dst_br,
            &skia_safe::Paint::default()
        );
    }
}