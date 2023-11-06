use std::time::SystemTime;
use nalgebra::Vector3;
use crate::freed::FreeD;
use crate::ptz::Ptz;
use crate::ndi::VideoData;
use crate::renderer::camera::Camera;
use crate::renderer::frame::Frame;

mod freed;
mod ndi;
mod ptz;
mod renderer;

fn main() {
    let bytes: [u8; 29] = [0xd1, 0x00, 0xdf, 0x78, 0xaa, 0x00, 0x47, 0xef,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x40, 0x00, 0x00, 0x15, 0x01, 0xd1, 0xff, 0x12];

    let bytes2: [u8; 29] = [0xd1, 0x00, 0x21, 0x98, 0xd8, 0xff, 0xeb, 0x94,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x28, 0xc6, 0x00, 0x13, 0xb9, 0xd1, 0xff, 0xd6];

    println!("{:?}", FreeD::try_from(&bytes));
    println!("{:?}", FreeD::try_from(&bytes2));

    //renderer::gpu::main();
    //return;
    Ptz::start_listening(5555);

    ndi::initialize().unwrap();

    println!("FreeD listener started and NDI library initialized.");

    if true {
        send_line();
    } else {
        send_test_video(Frame::new(1920, 1080, &mut vec![0u8; 1920 * 1080 * 3]).video_data);
    }
}

const LINE: (Vector3<f32>, Vector3<f32>) = (Vector3::new(0.0, -2.0, -10.0), Vector3::new(0.0, 0.0, -10.0));
const LINE2: (Vector3<f32>, Vector3<f32>) = (Vector3::new(-1.0, -1.0, -10.0), Vector3::new(1.0, -1.0, -10.0));

fn send_line() {
    let send = ndi::SendBuilder::new()
        .ndi_name("line overlay".to_string())
        .build()
        .unwrap();

    let mut buf = Vec::with_capacity((1920 * 1080 * 3) as usize);
    let mut frame = Frame::new(1920, 1080, &mut buf);

    let mut camera = Camera::default();
    loop {
        // Get the current time
        let start_time = SystemTime::now();

        let n_frames = 100;
        for _ in 0..n_frames {
            let (yaw, pitch, roll) = Ptz::yaw_pitch_roll();
            camera.set_rotation(yaw, pitch, roll);
            let p_0 = camera.project(LINE.0);
            let p_1 = camera.project(LINE.1);
            let p_2 = camera.project(LINE2.0);
            let p_3 = camera.project(LINE2.1);

            println!("yaw: {yaw:.2}°, pitch: {pitch:.2}°");

            // Clear all
            unsafe { frame.video_data.p_data().write_bytes(0, (frame.width() * frame.height() * 3) as usize); }

            frame.draw_line(p_0, p_1);
            frame.draw_line(p_2, p_3);

            // We now submit the frame. Note that this call will be clocked so that we end up submitting at exactly the specified frame rate.
            send.send_video(&frame.video_data);
        }

        // Get the end time
        let end_time = SystemTime::now();

        // Just display something helpful
        // printf("256 frames sent, average fps=%1.2f\n",
        //        256.0f / std::chrono::duration_cast<std::chrono::duration<float>>(end_time - start_time).count());
        println!("{n_frames} frames sent, average fps is {}", n_frames as f32 / end_time.duration_since(start_time).unwrap().as_secs_f32())
    }
}

fn send_test_video(video_frame: VideoData) {
    let send = ndi::SendBuilder::new()
        .ndi_name("test video".to_string())
        .build()
        .unwrap();

    let xres = video_frame.width();
    let yres = video_frame.height();

    loop {
        // Get the current time
        let start_time = SystemTime::now();

        // Send 256 frames
        for idx in 0..256 {
            // Fill in the buffer. It is likely that you would do something much smarter than this.

            let stride = video_frame.line_stride_in_bytes().unwrap();
            for y in 0..yres {
                for x in (0..xres).step_by(2) {
                    unsafe {
                        // Fill two pixels at once
                        *video_frame.p_data().offset((y * stride + x * 2 + 0) as isize) = (x * 256 / xres) as u8; // U
                        *video_frame.p_data().offset((y * stride + x * 2 + 1) as isize) = 127; // Y1
                        *video_frame.p_data().offset((y * stride + x * 2 + 2) as isize) = (idx + (y * 256 / yres)) as u8; // V
                        *video_frame.p_data().offset((y * stride + x * 2 + 3) as isize) = 127; // Y2
                    }
                }
            }

            //memset(videoFrame.p_data + stride * yres, 255, xres * yres * 2);

            let offset = stride * yres;
            let alpha_stride = video_frame.width();
            for y in 0..yres {
                for x in 0..xres {
                    unsafe {
                        *video_frame.p_data().offset((offset + y * alpha_stride + x) as isize) = ((idx + 70) + (x * 256 / xres)) as u8; // alpha
                    }
                }
            }

            // We now submit the frame. Note that this call will be clocked so that we end up submitting at exactly the specified frame rate.
            send.send_video(&video_frame);
        }

        // Get the end time
        let end_time = SystemTime::now();

        // Just display something helpful
        // printf("256 frames sent, average fps=%1.2f\n",
        //        256.0f / std::chrono::duration_cast<std::chrono::duration<float>>(end_time - start_time).count());
        println!("256 frames sent, average fps is {}", 256.0 / end_time.duration_since(start_time).unwrap().as_secs_f32())
    }
}
