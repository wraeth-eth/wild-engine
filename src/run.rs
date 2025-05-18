use super::state::State;
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
    let mut state = State::new(&window).await;
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
                if let Err(err) = state.window.set_cursor_grab(CursorGrabMode::Confined) {
                    log::warn!("Could not lock cursor: {err:?}");
                }
                state.window.set_cursor_visible(false);

                state.camera_controller.process_mouse(delta.0, delta.1);
            }
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == state.window().id() && !state.input(event) => match event {
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
                    state.resize(*physical_size);
                }
                WindowEvent::RedrawRequested => {
                    state.window().request_redraw();
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    state.update(dt);
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size)
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
    //     // match event {
    //     // Event::WindowEvent {
    //     //     window_id,
    //     //     ref event,
    //     // } if window_id == state.window.id() => {
    //     //     if !state.input(event) {
    //     //         match event {
    //     //             WindowEvent::CloseRequested
    //     //             | WindowEvent::KeyboardInput {
    //     //                 event:
    //     //                     KeyEvent {
    //     //                         state: ElementState::Pressed,
    //     //                         physical_key: PhysicalKey::Code(KeyCode::Escape),
    //     //                         ..
    //     //                     },
    //     //                 ..
    //     //             } => control_flow.exit(),
    //     //             WindowEvent::Resized(physical_size) => {
    //     //                 log::info!("physical_size: {physical_size:?}");
    //     //                 surface_configured = true;
    //     //                 state.resize(*physical_size);
    //     //             }
    //     //             WindowEvent::RedrawRequested => {
    //     //                 state.window().request_redraw();

    //     //                 if !surface_configured {
    //     //                     return;
    //     //                 }
    //     //                 state.update();
    //     //                 match state.render() {
    //     //                     Ok(_) => {}
    //     //                     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
    //     //                         state.resize(state.size)
    //     //                     }
    //     //                     Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
    //     //                         log::error!("OutOfMemory");
    //     //                         control_flow.exit();
    //     //                     }
    //     //                     Err(wgpu::SurfaceError::Timeout) => {
    //     //                         log::warn!("Surface timeout")
    //     //                     }
    //     //                 }
    //     //             }
    //     //             _ => {}
    //     //         }
    //     //     }
    //     // }
    //     _ => {}
    // });
}
