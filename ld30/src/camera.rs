
use piston::cam;
use piston::vecmath;

pub type Matrix4 = vecmath::Matrix4<f32>;

pub struct CameraManager {
    pub projection: Matrix4,
    pub first_person: cam::FirstPerson,
}

impl CameraManager {
    pub fn new() -> CameraManager {
        let projection = cam::CameraPerspective {
                fov: 70.0,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: 1.0
            }.projection();
        let first_person = cam::FirstPerson::new(
                0.0, 0.0, 0.0,
                cam::FirstPersonSettings::default()
            );
        CameraManager {
            projection: projection,
            first_person: first_person
        }
    }
}
