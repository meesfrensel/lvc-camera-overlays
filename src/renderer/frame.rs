use nalgebra::Point2;
use crate::ndi::{FourCCVideoType, FrameFormatType, VideoData};

pub struct Frame {
    pub video_data: VideoData,
}

impl Frame {
    pub fn new(width: u32, height: u32, buf: &mut Vec<u8>) -> Self {
        // let mut buf = vec![0_u8; (width * height * 3) as usize];
        // let mut buf = Vec::with_capacity((width * height * 3) as usize);

        let f = Frame {
            video_data: VideoData::from_buffer(
                width,
                height,
                FourCCVideoType::UYVA,
                30,
                1,
                FrameFormatType::Progressive,
                0,
                width * 2, // two bytes per pixel with UYVA
                None,
                buf.as_mut_slice(),
            )
        };
        unsafe { f.video_data.p_data().write_bytes(0, (width * height * 3) as usize); }
        f
    }

    pub fn clear(&mut self) {
        unsafe { self.video_data.p_data().write_bytes(0, (self.width() * self.height() * 2) as usize); }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, u: u8, v: u8, lum: u8, a: u8) {
        if x >= self.width() || y >= self.height() {
            // println!("out of range");
            return;
        }
        let stride= self.video_data.line_stride_in_bytes().unwrap();
        let alpha_plane = stride * self.video_data.height();
        let x_offset = 1 + (x % 2) * 2;
        let y_offset = y * stride;
        unsafe {
            *self.video_data.p_data().offset((y_offset + x * 2 + 0) as isize) = u; // U
            *self.video_data.p_data().offset((y_offset + x * 2 + 2) as isize) = v; // V
            *self.video_data.p_data().offset((y_offset + x * 2 + x_offset) as isize) = lum; // Y
            *self.video_data.p_data().offset((alpha_plane + y_offset / 2 + x) as isize) = a; // alpha
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

    pub fn width(&self) -> u32 {
        self.video_data.width()
    }

    pub fn height(&self) -> u32 {
        self.video_data.height()
    }
}
