// // https://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-01-window.html
// // https://medium.com/@aleksej.gudkov/rust-winit-example-creating-a-windowed-application-0eaaf395d776
// use winit::{
//     event::{Event, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     window::WindowBuilder,
// };
// extern crate gl;

// fn main() {
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .with_title("Winit Example")
//         .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
//         // .with_inner_size(winit::dpi::LogicalSize::new(800, 600))

//         .build(&event_loop)
//         .expect("Failed to create window");
//     println!("Window created {:?}", wextern crate gl;indow);

//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Wait;
//         match event {
//             Event::WindowEvent { event, .. } => match event {
//                 WindowEvent::CloseRequested => {
//                     println!("Window close requested");
//                     *control_flow = ControlFlow::Exit;
//                 }
//                 WindowEvent::Resized(size) => {
//                     println!("Window resized: {:?}", size);
//                 }
//                 WindowEvent::KeyboardInput { input, .. } => {
//                     println!("Keyboard input: {:?}", input);
//                 }
//                 WindowEvent::CursorMoved { position, .. } => {
//                     println!("Mouse moved to {:?}", position);
//                 }
//                 WindowEvent::MouseInput { state, button, .. } => {
//                     println!("Mouse button {:?} was {:?}", button, state);
//                 }
//                 _ => {}
//             },
//             Event::MainEventsCleared => {
//                 window.request_redraw();
//             }
//             Event::RedrawRequested(_) => {
//                 // println!("Redrawing the window");
//             }
//             _ => {}
//         }
//     });
// }

mod camera;
mod instance;
mod run;
mod state;
mod texture;

// https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#using-a-pipeline

fn main() {
    pollster::block_on(run::run());
    // env_logger::init();
    // let event_loop = EventLoop::new().unwrap();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();

    // let _ = event_loop.run(move |event, control_flow| match event {
    //     Event::WindowEvent {
    //         window_id,
    //         ref event,
    //     } if window_id == window.id() => match event {
    //         WindowEvent::CloseRequested
    //         | WindowEvent::KeyboardInput {
    //             event:
    //                 KeyEvent {
    //                     state: ElementState::Pressed,
    //                     physical_key: PhysicalKey::Code(KeyCode::Escape),
    //                     ..
    //                 },
    //             ..
    //         } => control_flow.exit(),
    //         _ => {}
    //     },
    //     _ => {}
    // });
}
