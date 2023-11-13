
fn main() {
    let config = mrgr::Config {
        viewport_size: (320, 240),
        zoom_level: 4,
        framerate: 60,
        caption: "Hello, World from MRGR!",
        game_main,
    };
    mrgr::launch(config);
}

fn game_main() {
    
}