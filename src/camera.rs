use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

/// Homogeneous matrix used to fix differences between OpenGL (Y-up, Z-forward)
/// and WGPU (Y-up, Z-back).
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

/// Contains the properties and methods relative to building a projection matrix
/// of the camera.
pub struct Camera {
    /// Where the camera is in world space.
    pub eye: cgmath::Point3<f32>,
    /// The direction that the camera is facing (where the camera is looking).
    pub target: cgmath::Point3<f32>,
    /// The direction that is 'up' with respect to the camera.
    pub up: cgmath::Vector3<f32>,
    /// The aspect ratio of the camera (width / height).
    pub aspect: f32,
    /// Vertical field of view (degrees).
    pub fovy: f32,
    /// The near clipping plane.
    pub znear: f32,
    /// The far clipping plane.
    pub zfar: f32,
}

impl Camera {
    /// Builds the view projection matrix for the `Camera` attributes.
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let projection =
            cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * projection * view;
    }
}

/// Raw representation of the camera. Bound directly with the shader code.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    /// Camera position in world space.
    pub view_position: [f32; 4],
    /// The view of the camera with the projection applied to it.
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_projection: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        // Using Vector4 because of uniforms 16 byte spacing requirement
        self.view_position = camera.eye.to_homogeneous().into();
        self.view_projection = camera.build_view_projection_matrix().into();
    }
}

/// Controls camera movements. Used to manipulate camera displacement and rotation.
pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_backward_pressed: false,
            is_forward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    // TODO: Properly understand and optimise this code.
    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_normalized = forward.normalize();
        let forward_magnitude = forward.magnitude();

        // Prevents glitch when camera gets too close to center of scene.
        if self.is_forward_pressed && forward_magnitude > self.speed {
            camera.eye += forward_normalized * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_normalized * self.speed;
        }

        let right = forward_normalized.cross(camera.up);

        // Recalculate origin for next calcs
        let forward = camera.target - camera.eye;
        let forward_magnitude = forward.magnitude();

        if self.is_right_pressed {
            camera.eye =
                camera.target - (forward + right * self.speed).normalize() * forward_magnitude;
        }
        if self.is_left_pressed {
            camera.eye =
                camera.target - (forward - right * self.speed).normalize() * forward_magnitude;
        }
    }
}
