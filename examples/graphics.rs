use std::thread;
use std::time::Duration;
use mrgr::*;

fn main() {
    let config = MrgrConfig {
        viewport_size: (320, 240),
        zoom_level: 4,
        framerate: 30,
        caption: "Graphics in MRGR",
        game_main,
    };
    mrgr_launch(config);
}

fn game_main() {
    load_bitmap_font("../mrgr-fontgen/noto_sans_mono_cjk_sc");

    let mut ratio = 0.20;
    let (cx, cy) = (160, 175);
    let (gx, gy) = (150, 55);
    while ratio < 1.0 {
        paint(move |mut canvas| {
            let (x, y) = (cx as f64 - gx as f64 * ratio, cy as f64 - gy as f64 * ratio);
            let (w, h) = (gx as f64 * ratio * 2.0, gy as f64 * ratio * 2.0);
            canvas.nine_patch("asset/nine_patch.png", NinePatch::from_uni(6),
                              (x as i32, y as i32), (w as i32, h as i32));
        });
        thread::sleep(Duration::from_millis(1000 / 30));
        ratio += 0.1
    }

    paint(|mut canvas| {
        canvas.nine_patch("asset/nine_patch.png", NinePatch::from_uni(6), (10, 120), (300, 110));
        canvas.text("This is some text in a nine-patch\nimage!", TextStyle::SmallNormal, (20, 130), 280);
    });
}