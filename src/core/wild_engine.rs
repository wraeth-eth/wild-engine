use std::time::Duration;

use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

use super::{graphics::camera, state::State};

// TODO: Maybe eventually write a config?
// pub struct Config {
//     backends: wgpu::Backends,
// }

// impl Default for Config {
//     fn default() -> Self {
//         Self {
//             backends: wgpu::Backends::default(),
//         }
//     }
// }

// pub struct ConfigBuilder {
//     inner: Config,
// }

// impl ConfigBuilder {
//     pub fn new() -> Self {
//         let inner = Config::default();
//         Self { inner }
//     }

//     pub fn backends(mut self, backends: wgpu::Backends) -> Self {
//         self.inner.backends = backends;
//         self
//     }
// }

pub struct WildEngine<'a> {
    state: State<'a>,
}

impl<'a> WildEngine<'a> {
    pub async fn new(window: &'a Window) -> WildEngine<'a> {
        let state = State::new(window).await;
        Self { state }
    }

    pub fn window(&self) -> &Window {
        self.state.window()
    }

    pub fn resize_to_current(&mut self) {
        self.state.resize(self.state.size);
    }

    pub fn resize_to(&mut self, new_size: PhysicalSize<u32>) {
        self.state.resize(new_size);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.state.input(event)
    }

    pub fn update(&mut self, dt: Duration) {
        self.state.update(dt);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.state.render()
    }

    pub fn camera_controller(&mut self) -> &mut camera::CameraController {
        &mut self.state.camera_controller
    }
}
