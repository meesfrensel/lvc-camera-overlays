use std::sync::Mutex;
use std::time::Duration;
use crate::freed::FreeD;

static CURRENT_FREED_DATA: Mutex<FreeD> = Mutex::new(FreeD::zero());

pub struct Ptz;

impl Ptz {
    pub fn start_listening(port: u16) {
        std::thread::spawn(move || {
            let socket = std::net::UdpSocket::bind(("0.0.0.0", port)).unwrap();
            let mut buf = [0u8; 29];

            loop {
                if let Ok((amount, _source)) = socket.recv_from(&mut buf) {
                    if amount == 29 {
                        if let Ok(freed) = FreeD::try_from(&buf) {
                            //println!("{freed:?}");
                            Self::set_freed_data(freed);
                        } else {
                            println!("error")
                        }
                    } else {
                        println!("{:?}", buf);
                    }
                } else {
                    println!("err");
                }

                std::thread::sleep(Duration::from_millis(1));
            }
        });
    }

    pub fn set_freed_data(freed: FreeD) {
        *CURRENT_FREED_DATA.lock().unwrap() = freed;
    }

    pub fn yaw_pitch_zoom() -> (f32, f32, u32) {
        let data = CURRENT_FREED_DATA.lock().unwrap();

        (data.tilt, data.pan, data.zoom)
    }
}