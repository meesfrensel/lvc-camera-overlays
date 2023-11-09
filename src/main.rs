use std::time::SystemTime;
use nalgebra::Vector3;
use crate::ptz::Ptz;
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

const LINE: (Vector3<f32>, Vector3<f32>) = (Vector3::new(0.05, -0.36, -1.8), Vector3::new(0.05, 0.0, -1.8));
const LINE2: (Vector3<f32>, Vector3<f32>) = (Vector3::new(-0.13, -0.18, -1.8), Vector3::new(0.23, -0.18, -1.8));
const WB_0: (Vector3<f32>, Vector3<f32>) = (Vector3::new(-0.6, 0.11, -1.8), Vector3::new(0.6, 0.11, -1.8));
const WB_1: (Vector3<f32>, Vector3<f32>) = (Vector3::new(-0.6, -0.79, -1.8), Vector3::new(0.6, -0.79, -1.8));

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
            let (yaw, pitch, zoom) = Ptz::yaw_pitch_zoom();
            camera.set_rotation(yaw, pitch, 0.0);
            camera.set_zoom(zoom);
            let p_0 = camera.project(LINE.0);
            let p_1 = camera.project(LINE.1);
            let p_2 = camera.project(LINE2.0);
            let p_3 = camera.project(LINE2.1);
            let p_4 = camera.project(WB_0.0);
            let p_5 = camera.project(WB_0.1);
            let p_6 = camera.project(WB_1.0);
            let p_7 = camera.project(WB_1.1);

            println!("yaw: {yaw:.2}°, pitch: {pitch:.2}°, zoom: {zoom}");

            frame.clear();

            frame.draw_thick_line(p_0, p_1, 8.0, (255, 127, 127));
            frame.draw_thick_line(p_2, p_3, 8.0, (255, 127, 127));
            frame.draw_thick_line(p_4, p_5, 4.0, (135, 84, 73));
            frame.draw_thick_line(p_6, p_7, 4.0, (135, 84, 73));
            frame.draw_thick_line(p_4, p_6, 4.0, (135, 84, 73));
            frame.draw_thick_line(p_5, p_7, 4.0, (135, 84, 73));
            frame.fill_circle(p_4.x as u32, p_4.y as u32, 170, 170, 135, 255);
            frame.fill_circle(p_5.x as u32, p_5.y as u32, 170, 170, 135, 255);
            frame.fill_circle(p_6.x as u32, p_6.y as u32, 170, 170, 135, 255);
            frame.fill_circle(p_7.x as u32, p_7.y as u32, 170, 170, 135, 255);

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
