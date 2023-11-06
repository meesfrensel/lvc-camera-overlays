use std::f32::consts::PI;
use nalgebra::{Matrix3x4, Matrix4, Point2, Vector3};

/// Focal length.
// const F: f32 = 1000.0;
// const F: f32 = 10.0 / 5.2 * 0.5 * 1920.0;
const F: f32 = 5.2 / 4.53219 * 1920.0;

/// Camera intrinsic parameters:
/// ```text
/// / α 0 c_x \
/// | 0 β c_y |
/// \ 0 0  1  /
/// ```
///
/// For example, c_x could be aspect_ratio * 0.5135 = 16 / 9 * 0.5135. This
/// results in perspective scaling based on z value (distance from camera).
const INTRINSIC_PARAMS: Matrix3x4<f32> = Matrix3x4::new(
    F * 0.73, 0.0, 0.0 /*16.0 / 9.0 * 0.5135*/, 0.0,
    0.0, F * 0.77, 0.0 /*9.0 / 16.0 * 0.5135*/, 0.0, // TODO: check
    0.0, 0.0, 1.0, 0.0,
);

/// Camera model. Does not by default include support for translations.
#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {
    /// Aka camera extrinsic matrix
    rotation_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32, roll: f32) {
        // In camera coordinate system, y is up/down and z is the original y.
        // Therefore, roll and yaw must be switched.
        self.rotation_matrix = Matrix4::from_euler_angles(yaw * PI / 180.0, pitch * PI / 180.0, roll * PI / 180.0);
    }

    /// Project a vector `[ x y z ]` into camera space coordinates.
    ///
    /// In the resulting coordinate system, the `z` axis looks into the camera,
    /// with higher `z` meaning an object is closer, and lower (or more
    /// negative) `z` being further away from the camera.
    pub fn project(&self, point: Vector3<f32>) -> Point2<f32> {
        let p: Vector3<f32> = INTRINSIC_PARAMS * self.rotation_matrix * point.insert_row(3, 1.0);

        Point2::new(
            -(p.x / p.z) + 960.0,
            -(p.y / p.z) + 540.0,
        )
    }
}

mod test {
    use super::*;

    #[test]
    fn test_test() {
        let mut cam = Camera::default();
        cam.set_rotation(0.0, 0.0, 0.0);
        println!("{}", cam.project(Vector3::new(1.5, 0.1, -2.0)));
        println!("{}", cam.project(Vector3::new(1.5, 1.0, -2.0)));
    }
}
