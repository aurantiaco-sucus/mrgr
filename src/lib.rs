mod config;
mod graphics;
mod runtime;
mod event;
mod resource;
mod common;

use log::{error, info};

pub use crate::config::MrgrConfig;
pub use crate::graphics::*;
pub use crate::event::*;
pub use crate::resource::*;

pub fn mrgr_launch(config: MrgrConfig) {
    env_logger::init();
    info!("Midnight233's Retro Game Renderer, Version {}", env!("CARGO_PKG_VERSION"));
    if !config.ensure_sanity() {
        error!("Configuration is not sane, aborting!");
        return;
    }
    runtime::launch_softbuffer_skia_raster_runtime(config);
}