use nalgebra::Point2;
use crate::ndi::{FourCCVideoType, FrameFormatType, VideoData};

pub struct Frame {
    pub video_data: VideoData,
}

impl Frame {
    pub fn new(width: u32, height: u32, buf: &mut Vec<u8>) -> Self {
        // let mut buf = vec![0_u8; (width * height * 3) as usize];
        // let mut buf = Vec::with_capacity((width * height * 3) as usize);

        Frame {
            video_data: VideoData::from_buffer(
                width,
                height,
                FourCCVideoType::UYVA,
                60,
                1,
                FrameFormatType::Progressive,
                0,
                width * 2, // two bytes per pixel with UYVA
                None,
                buf.as_mut_slice(),
            )
        }
    }

    pub fn clear(&mut self) {
        unsafe { self.video_data.p_data().write_bytes(0, (self.width() * self.height() * 3) as usize); }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, u: u8, v: u8, lum: u8, a: u8) {
        if x >= self.width() || y >= self.height() {
            // println!("out of range");
            return;
        }
        let stride= self.video_data.line_stride_in_bytes().unwrap();
        let uyvy_plane_offset = y * stride + (x / 2) * 4;
        let x_offset = (x % 2) * 2;
        let alpha_plane = stride * self.video_data.height();
        unsafe {
            *self.video_data.p_data().offset((uyvy_plane_offset + 0) as isize) = u; // U
            *self.video_data.p_data().offset((uyvy_plane_offset + 1 + x_offset) as isize) = lum; // Y
            *self.video_data.p_data().offset((uyvy_plane_offset + 2) as isize) = v; // V
            *self.video_data.p_data().offset((alpha_plane + y * stride / 2 + x) as isize) = a; // alpha
        }
    }

    pub fn fill_circle(&mut self, x: u32, y: u32, u: u8, v: u8, lum: u8, a: u8) {
        for xi in x-4..x+4 {
            for yi in y-4..y+4 {
                self.set_pixel(xi, yi, u, v, lum, a);
            }
        }
    }

    pub fn draw_line(&mut self, p0: Point2<f32>, p1: Point2<f32>) {
        if (p1.y - p0.y).abs() < (p1.x - p0.x).abs() {
            let (p0, p1) = if p0.x > p1.x {
                (p1, p0) // swap
            } else {
                (p0, p1)
            };
            self.draw_line_low(p0, p1);
        } else {
            let (p0, p1) = if p0.y > p1.y {
                (p1, p0) // swap
            } else {
                (p0, p1)
            };
            self.draw_line_high(p0, p1);
        }
    }

    pub fn draw_line_low(&mut self, p0: Point2<f32>, p1: Point2<f32>) {
        let dx = p1.x - p0.x;
        let mut dy = p1.y - p0.y;
        let mut yi = 1.0;
        if dy < 0.0 {
            yi = -1.0;
            dy = -dy;
        }
        let mut d = (2.0 * dy) - dx;
        let mut y = p0.y;

        for x in p0.x as i32..p1.x as i32 {
            self.set_pixel(x as u32, y as u32, 127, 127, 255, 255);
            if d > 0.0 {
                y += yi;
                d += 2.0 * (dy - dx);
            } else {
                d += 2.0 * dy;
            }
        }
    }

    pub fn draw_line_high(&mut self, p0: Point2<f32>, p1: Point2<f32>) {
        let mut dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let mut xi = 1.0;
        if dx < 0.0 {
            xi = -1.0;
            dx = -dx;
        }
        let mut d = (2.0 * dx) - dy;
        let mut x = p0.x;

        for y in p0.y as i32..p1.y as i32 {
            self.set_pixel(x as u32, y as u32, 127, 127, 255, 255);
            if d > 0.0 {
                x += xi;
                d += 2.0 * (dx - dy);
            } else {
                d += 2.0 * dx;
            }
        }
    }

    pub fn draw_thick_line(&mut self, p0: Point2<f32>, p1: Point2<f32>, width: f32, (lum, u, v): (u8, u8, u8)) {
        if width == 0.0 {
            return;
        }

        let (mut p0, mut p1) = (p0, p1);

        // Steep means that the slope is >1
        let steep = (p1.y - p0.y).abs() > (p1.x - p0.x).abs();
        // If steep, swap x and y to ensure dx > dy
        if steep {
            p0 = p0.yx();
            p1 = p1.yx();
        }

        // Swap endpoints to ensure that dx > 0
        if p0.x > p1.x {
            (p0, p1) = (p1, p0);
        }

        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let gradient = if dx > 0.0 { dy / dx } else { 1.0 };
        // Rotate width
        let w = width * (1.0 + gradient * gradient).sqrt();

        let (x_pixel_1, mut inter_y) =
            self.draw_thick_line_endpoint(p0, w, (lum, u, v), gradient, steep);
        let (x_pixel_2, _) =
            self.draw_thick_line_endpoint(p1, w, (lum, u, v), gradient, steep);
        let w = w as u32;

        for x in x_pixel_1 + 1..x_pixel_2 {
            let f_part = inter_y - inter_y.floor();
            let rf_part = 1.0 - f_part;
            let y = inter_y.floor() as u32;

            if steep {
                self.set_pixel(y, x, u, v, lum, (rf_part * 255.0) as u8);
                for i in 1..w { self.set_pixel(y + i, x, u, v, lum, 255); }
                self.set_pixel(y + w, x, u, v, lum, (f_part * 255.0) as u8);
            } else {
                self.set_pixel(x, y, u, v, lum, (rf_part * 255.0) as u8);
                for i in 1..w { self.set_pixel(x, y + i, u, v, lum, 255); }
                self.set_pixel(x, y + w, u, v, lum, (f_part * 255.0) as u8);
            }

            inter_y += gradient;
        }
    }

    fn draw_thick_line_endpoint(&mut self, p: Point2<f32>, w: f32, (y, u, v): (u8, u8, u8), gradient: f32, steep: bool) -> (u32, f32) {
        // First endpoint
        let x_end = p.x.round();
        let y_end = p.y - (w - 1.0) * 0.5 + gradient * (x_end - p.x);
        let x_gap = 1.0 - (p.x + 0.5 - x_end);
        let x_pixel = x_end as u32;
        let y_pixel = y_end.floor() as u32;
        let f_part = y_end - y_end.floor();
        let rf_part = 1.0 - f_part;
        let w = w as u32;

        if steep {
            self.set_pixel(y_pixel, x_pixel, u, v, y, (rf_part * x_gap) as u8);
            for i in 1..w { self.set_pixel(y_pixel + i, x_pixel, u, v, y, 255); }
            self.set_pixel(y_pixel + w, x_pixel, u, v, y, (rf_part * x_gap) as u8);
        } else {
            self.set_pixel(x_pixel, y_pixel, u, v, y, (rf_part * x_gap) as u8);
            for i in 1..w { self.set_pixel(x_pixel, y_pixel + i, u, v, y, 255); }
            self.set_pixel(x_pixel, y_pixel + w, u, v, y, (rf_part * x_gap) as u8);
        }

        (x_pixel, y_end + gradient)
    }

    pub fn width(&self) -> u32 {
        self.video_data.width()
    }

    pub fn height(&self) -> u32 {
        self.video_data.height()
    }
}
