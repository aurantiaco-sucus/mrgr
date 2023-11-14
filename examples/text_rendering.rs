use mrgr::*;

fn main() {
    let config = MrgrConfig {
        viewport_size: (320, 240),
        zoom_level: 4,
        framerate: 60,
        caption: "Text Rendering in MRGR",
        game_main,
    };
    mrgr_launch(config);
}

fn game_main() {
    load_bitmap_font("../mrgr-fontgen/noto_sans_mono_cjk_sc");
    paint(|mut canvas| {
        canvas.text("\
        This is a string like \"Hello, World!\", but I 想要 it to span across multiple lines, so I a\
        dded these 没有意思的 description around it.\
        ", TextStyle::SmallNormal, (0, 0), 320);
    });
}