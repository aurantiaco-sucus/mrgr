use mrgr::*;

fn main() {
    let config = MrgrConfig {
        viewport_size: (320, 240),
        zoom_level: 4,
        framerate: 60,
        caption: "Hello, World from MRGR!",
        game_main,
    };
    mrgr_launch(config);
}

fn game_main() {

}