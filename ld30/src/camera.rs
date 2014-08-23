
use piston::cam;
use piston::vecmath;

pub type Matrix4 = vecmath::Matrix4<f32>;

pub struct CameraManager {
    pub projection: Matrix4,
    pub camera: cam::Camera,
    fps_controller: cam::FPSController,
}

impl CameraManager {
    pub fn new() -> CameraManager {
        let projection = cam::CameraPerspective {
                fov: 70.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: 1.0
            }.projection();
        let fps_controller = cam::FPSController::new(
                cam::FPSControllerSettings::default()
            );
        let mut camera = cam::Camera::new(0.0, 0.0, 0.0);
        camera.set_yaw_pitch(fps_controller.yaw, fps_controller.pitch);
        CameraManager {
            projection: projection,
            camera: camera,
            fps_controller: fps_controller
        }
    }
}
