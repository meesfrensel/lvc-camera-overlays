use std::f32::consts::PI;
use nalgebra::{Matrix3x4, Matrix4, Point2, Vector3};

// From OpenCV. In pixels
const CENTER: (f32, f32) = (954.293667, 551.196783);
const F: (f32, f32) = (-1667.75409, -1670.73857);

/// Camera intrinsic parameters:
/// ```text
/// / α 0 c_x \
/// | 0 β c_y |
/// \ 0 0  1  /
/// ```
const INTRINSIC_PARAMS: Matrix3x4<f32> = Matrix3x4::new(
    F.0, 0.0, CENTER.0, 0.0,
    0.0, F.1, CENTER.1, 0.0,
    0.0, 0.0, 1.0, 0.0,
);

/// From OpenCV: k_1, k_2, p_1, p_2, k_3
const DISTORTION: [f32; 5] = [-0.09120233, 0.10029151, -0.0004659, -0.00094341, -0.05962273];

/// Camera model. Does not by default include support for translations.
#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {
    /// Aka camera extrinsic matrix
    rotation_matrix: Matrix4<f32>,
    focal_length: f32,
}

impl Camera {
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32, roll: f32) {
        // In camera coordinate system, y is up/down and z is the original y.
        // Therefore, roll and yaw must be switched.
        self.rotation_matrix = Matrix4::from_euler_angles(yaw * PI / 180.0, pitch * PI / 180.0, roll * PI / 180.0);
    }

    pub fn set_zoom(&mut self, zoom: u32) {
        self.focal_length = match zoom {
            0..=4095 => 1.0 + zoom as f32 / 4096.0 * 0.6,
            4096..=16384 => 1.6 + (zoom - 4096) as f32 / 16384.0 * 5.0,
            _ => panic!("zoom out of range")
        }
    }

    /// Project a vector `[ x y z ]` into camera space coordinates.
    ///
    /// In the resulting coordinate system, the `z` axis looks into the camera,
    /// with higher `z` meaning an object is closer, and lower (or more
    /// negative) `z` being further away from the camera.
    pub fn project(&self, point: Vector3<f32>) -> Point2<f32> {
        let zoom_matrix = Matrix4::new(self.focal_length, 0.0, 0.0, 0.0, 0.0, self.focal_length, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        //zoom_matrix.fill_with_identity();
        let p: Vector3<f32> = INTRINSIC_PARAMS * zoom_matrix * self.rotation_matrix * point.insert_row(3, 1.0);
        let x = ((p.x / p.z) - CENTER.0) / F.0;
        let y = ((p.y / p.z) - CENTER.1) / F.1;
        let r2 = x * x + y * y;
        let x_distorted = x * (1.0 + DISTORTION[0] * r2 + DISTORTION[1] * r2.powi(2) + DISTORTION[4] * r2.powi(3))+ 2.0 * DISTORTION[2] * x * y + DISTORTION[3] * (r2 + 2.0 * x * x);
        let y_distorted = y * (1.0 + DISTORTION[0] * r2 + DISTORTION[1] * r2.powi(2) + DISTORTION[4] * r2.powi(3))+ DISTORTION[3] * (r2 + 2.0 * y * y) + 2.0 * DISTORTION[3] * x * y;

        Point2::new(x_distorted * F.0 + CENTER.0, y_distorted * F.1 + CENTER.1)
        // Point2::new(p.x / 100.0 + 960.0, p.y / 100.0 + 540.0)
    }
}

mod test {
    #[test]
    fn test_test() {
        let mut cam = Camera::default();
        cam.set_rotation(0.0, 0.0, 0.0);
        println!("{}", cam.project(Vector3::new(1.5, 0.1, -2.0)));
        println!("{}", cam.project(Vector3::new(1.5, 1.0, -2.0)));
    }
}
