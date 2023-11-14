use winit::dpi::PhysicalSize;
use log::error;

pub const MAXIMUM_WIDTH: u32 = 1280;
pub const MAXIMUM_HEIGHT: u32 = 720;


pub const MINIMUM_WIDTH: u32 = 128;
pub const MINIMUM_HEIGHT: u32 = 72;

pub const MAXIMUM_ZOOM_LEVEL: u32 = 10;
pub const MINIMUM_ZOOM_LEVEL: u32 = 1;

pub const MAXIMUM_FRAMERATE: u32 = 480;
pub const MINIMUM_FRAMERATE: u32 = 15;

pub const fn minimum_physical_size() -> PhysicalSize<u32> {
    PhysicalSize::new(MINIMUM_WIDTH * MINIMUM_ZOOM_LEVEL,
                      MINIMUM_HEIGHT * MINIMUM_ZOOM_LEVEL)
}

pub const fn maximum_physical_size() -> PhysicalSize<u32> {
    PhysicalSize::new(MAXIMUM_WIDTH * MAXIMUM_ZOOM_LEVEL,
                      MAXIMUM_HEIGHT * MAXIMUM_ZOOM_LEVEL)
}

pub struct MrgrConfig {
    pub viewport_size: (u32, u32),
    pub zoom_level: u32,
    pub framerate: u32,
    pub caption: &'static str,
    pub game_main: fn(),
}

impl MrgrConfig {
    pub(crate) fn ensure_sanity(&self) -> bool {
        if (self.viewport_size.0 > MAXIMUM_WIDTH) || (self.viewport_size.1 > MAXIMUM_HEIGHT) {
            error!("Wrong configuration: viewport size is too big!");
            return false;
        }
        if (self.viewport_size.0 < MINIMUM_WIDTH) || (self.viewport_size.1 < MINIMUM_HEIGHT) {
            error!("Wrong configuration: viewport size is too small!");
            return false;
        }
        if (self.zoom_level > MAXIMUM_ZOOM_LEVEL) || (self.zoom_level < MINIMUM_ZOOM_LEVEL) {
            error!("Wrong configuration: zoom level is too big or too small!");
            return false;
        }
        if (self.framerate > MAXIMUM_FRAMERATE) || (self.framerate < MINIMUM_FRAMERATE) {
            error!("Wrong configuration: framerate is too hig or too low!");
            return false;
        }
        true
    }

    pub(crate) fn physical_width(&self) -> u32 {
        self.viewport_size.0 * self.zoom_level
    }

    pub(crate) fn physical_height(&self) -> u32 {
        self.viewport_size.1 * self.zoom_level
    }

    pub(crate) fn physical_size(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.physical_width(), self.physical_height())
    }
}