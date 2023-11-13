mod config;
mod graphics;
mod runtime;
mod event;
mod resource;
mod typography;
mod common;

use log::{error, info};

pub use crate::config::LaunchConfig;
pub use crate::graphics::*;
pub use crate::event::*;
pub use crate::resource::*;
pub use crate::typography::*;

pub fn launch(config: LaunchConfig) {
    env_logger::init();
    info!("Midnight233's Retro Game Renderer, Version {}", env!("CARGO_PKG_VERSION"));
    if !config.ensure_sanity() {
        error!("Configuration is not sane, aborting!");
        return;
    }
    runtime::launch_softbuffer_skia_raster_runtime(config);
}