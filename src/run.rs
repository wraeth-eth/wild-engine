use crate::core;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut wild_engine = core::WildEngine::new(&window).await;
    let mut last_render_time = instant::Instant::now();

    let _ = event_loop
        .run(move |event, control_flow| match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                println!("Mouse mov");
                // TODO: might not be best place for this
                // Lock mouse
                use winit::window::CursorGrabMode;
                if let Err(err) = wild_engine
                    .window()
                    .set_cursor_grab(CursorGrabMode::Confined)
                {
                    log::warn!("Could not lock cursor: {err:?}");
                }
                wild_engine.window().set_cursor_visible(false);
                wild_engine
                    .camera_controller()
                    .process_mouse(delta.0, delta.1);
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == wild_engine.window().id() && !wild_engine.input(event) => match event
            {
                #[cfg(not(target_arch = "wasm32"))]
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                WindowEvent::Resized(physical_size) => {
                    wild_engine.resize_to(*physical_size);
                }
                WindowEvent::RedrawRequested => {
                    wild_engine.window().request_redraw();
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    wild_engine.update(dt);
                    match wild_engine.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            wild_engine.resize_to_current()
                        }
                        Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                            log::error!("OutOfMemory");
                            control_flow.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timeout")
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
