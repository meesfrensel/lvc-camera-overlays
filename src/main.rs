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
    Ptz::start_listening(5555);

    ndi::initialize().unwrap();

    println!("FreeD listener started and NDI library initialized.");

    send_line();
}

const LINE: (Vector3<f32>, Vector3<f32>) = (Vector3::new(0.0, -2.0, -10.0), Vector3::new(0.0, 0.0, -10.0));
const LINE2: (Vector3<f32>, Vector3<f32>) = (Vector3::new(-1.0, -1.0, -10.0), Vector3::new(1.0, -1.0, -10.0));
const LINE3: (Vector3<f32>, Vector3<f32>) = (Vector3::new(-2.0, 1.0, -10.0), Vector3::new(2.0, 0.0, -10.0));

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
            let p_4 = camera.project(LINE3.0);
            let p_5 = camera.project(LINE3.1);

            println!("yaw: {yaw:.2}°, pitch: {pitch:.2}°");

            frame.clear();

            frame.draw_thick_line(p_0, p_1, 8.0, (255, 127, 127));
            frame.draw_thick_line(p_2, p_3, 8.0, (255, 127, 127));
            frame.draw_thick_line(p_4, p_5, 8.0, (255, 127, 127));

            frame.draw_line(nalgebra::Point2::new(200.0, 200.0), nalgebra::Point2::new(200.0, 300.0));
            frame.draw_line(nalgebra::Point2::new(203.0, 250.0), nalgebra::Point2::new(203.0, 350.0));
            frame.draw_line(nalgebra::Point2::new(250.0, 200.0), nalgebra::Point2::new(250.0, 300.0));
            frame.draw_line(nalgebra::Point2::new(252.0, 250.0), nalgebra::Point2::new(252.0, 350.0));
            frame.draw_line(nalgebra::Point2::new(300.0, 200.0), nalgebra::Point2::new(300.0, 300.0));
            frame.draw_line(nalgebra::Point2::new(301.0, 250.0), nalgebra::Point2::new(301.0, 350.0));

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
