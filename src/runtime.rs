use std::{slice, thread};
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;
use std::time::{Duration, Instant};
use winit::event::{DeviceEvent, Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
use winit::window::{Window, WindowBuilder, WindowButtons};
use crate::{Config, config};

static mut GAME_THREAD: Option<ThreadId> = None;

pub struct Runtime {
    pub proxy: EventLoopProxy<RuntimeEvent>,
    pub surface: Arc<Mutex<skia_safe::Surface>>,
}

static mut RT: Option<Runtime> = None;

pub fn runtime() -> &'static Runtime {
    unsafe { RT.as_ref().unwrap() }
}

pub enum RuntimeEvent {
    ShutdownRequested
}

fn winit_create_window(el: EventLoop<RuntimeEvent>, config: &Config) -> Window {
    WindowBuilder::new()
        .with_active(true)
        .with_inner_size(config.physical_size())
        .with_min_inner_size(config::minimum_physical_size())
        .with_max_inner_size(config::maximum_physical_size())
        .with_resizable(false)
        .with_title(config.caption.clone())
        .with_enabled_buttons(WindowButtons::CLOSE)
        .build(&el).unwrap()
}

fn softbuffer_create_context<'a>(
    config: &Config,
    window: &'a Window
) -> softbuffer::Surface<&'a Window, &'a Window> {
    let context = softbuffer::Context::new(window).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window).unwrap();
    surface.resize(
        config.physical_width().try_into().unwrap(),
        config.physical_height().try_into().unwrap()
    ).unwrap();
    surface
}

pub fn skia_softbuffer_surface<'pixels>(
    config: &Config,
    softbuffer_surface: &mut softbuffer::Surface<&Window, &Window>,
) -> skia_safe::Borrows<'pixels, skia_safe::Surface> {
    let size = config.physical_size();
    let buf_ptr = softbuffer_surface.buffer_mut().unwrap().as_mut_ptr() as *mut u8;
    let pixel_count = size.width as usize * size.height as usize;
    let buf = unsafe { slice::from_raw_parts_mut(buf_ptr, pixel_count * 4) };
    let info = skia_safe::ImageInfo::new(
        (size.width as i32, size.height as i32),
        skia_safe::ColorType::BGRA8888,
        skia_safe::AlphaType::Opaque,
        skia_safe::ColorSpace::new_srgb()
    );
    skia_safe::surfaces::wrap_pixels(
        &info, buf, size.width as usize * 4, None).unwrap()
}

pub fn skia_raster_surface<'pixels>(config: &Config) -> skia_safe::Surface {
    skia_safe::surfaces::raster_n32_premul(
        (config.viewport_size.0 as i32, config.viewport_size.1 as i32)).unwrap()
}

pub fn launch_softbuffer_skia_raster_runtime(config: Config) {
    let event_loop = EventLoopBuilder::<RuntimeEvent>::with_user_event()
        .build().unwrap();
    let proxy = event_loop.create_proxy();
    let window = winit_create_window(event_loop, &config);

    let mut sb_surface = softbuffer_create_context(&config, &window);
    let mut present_surface = skia_softbuffer_surface(&config, &mut sb_surface);
    let mut viewport_surface = Arc::new(Mutex::new(skia_raster_surface(&config)));

    let rt = Runtime {
        proxy,
        surface: viewport_surface.clone(),
    };
    unsafe { RT = Some(rt) }

    let dur_frame = Duration::from_micros(1_000_000 / config.framerate as u64);
    let mut ins_last_frame = Instant::now() - dur_frame;

    let present_bounds = {
        let p_width = config.physical_width();
        let p_height = config.physical_height();
        skia_safe::Rect::new(0.0, 0.0, p_width as f32, p_height as f32)
    };
    let default_paint = skia_safe::Paint::default();

    event_loop.run(move |event, window_target| {
        match event {
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => {}
                StartCause::WaitCancelled { .. } => {}
                StartCause::Poll => {}
                StartCause::Init => {
                    let jh = thread::spawn(move || {
                        thread::park();
                        (config.game_main)();
                    });
                    unsafe { GAME_THREAD = Some(jh.thread().id()); }
                    jh.thread().unpark();
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Moved(_) => {}
                WindowEvent::CloseRequested => {}
                WindowEvent::Destroyed => {}
                WindowEvent::DroppedFile(_) => {}
                WindowEvent::HoveredFile(_) => {}
                WindowEvent::HoveredFileCancelled => {}
                WindowEvent::Focused(_) => {}
                WindowEvent::KeyboardInput { event, .. } => {
                    let logical = event.logical_key;
                }
                WindowEvent::ModifiersChanged(_) => {}
                WindowEvent::Ime(_) => {}
                WindowEvent::CursorMoved { .. } => {}
                WindowEvent::CursorEntered { .. } => {}
                WindowEvent::CursorLeft { .. } => {}
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::MouseInput { .. } => {
                }
                WindowEvent::AxisMotion { .. } => {}
                WindowEvent::Touch(_) => {}
                WindowEvent::Occluded(_) => {}
                WindowEvent::RedrawRequested => {
                    ins_last_frame = Instant::now() - dur_frame;
                }
                _ => {}
            }
            Event::DeviceEvent { device_id, event } => match event {
                DeviceEvent::Added => {}
                DeviceEvent::Removed => {}
                DeviceEvent::MouseMotion { .. } => {}
                DeviceEvent::MouseWheel { .. } => {}
                DeviceEvent::Motion { .. } => {}
                DeviceEvent::Button { .. } => {}
                DeviceEvent::Key(_) => {}
            }
            Event::UserEvent(mrgr_event) => {}
            Event::LoopExiting => {}
            _ => {}
        }
        if ins_last_frame.elapsed() >= dur_frame {
            ins_last_frame += dur_frame;
            let image = viewport_surface.lock().unwrap().image_snapshot();
            present_surface.canvas()
                .draw_image_rect(&image, None, &present_bounds, &default_paint);
            sb_surface.buffer_mut().unwrap().present().unwrap();
        }
        window_target.set_control_flow(ControlFlow::WaitUntil(ins_last_frame + dur_frame))
    }).unwrap();
}