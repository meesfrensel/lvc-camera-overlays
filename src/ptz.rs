use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crate::freed::FreeD;

pub struct Ptz {
    num: u8,
    latest_freed_data: Arc<Mutex<FreeD>>,
}

impl Ptz {
    /// Bind UDP socket to this address. 0.0.0.0 resolves to own address.
    const BASE_ADDRESS: &'static str = "0.0.0.0";
    const BASE_PORT: u16 = 5550;

    pub fn new(ptz_num: u8) -> Self {
        assert!(ptz_num <= 6, "LVC heeft maar 6 PTZs");
        Ptz {
            num: ptz_num,
            latest_freed_data: Arc::new(Mutex::new(FreeD::zero())),
        }
    }

    pub fn start_listening(self, running: Arc<AtomicBool>) -> Self {
        let port = Self::BASE_PORT + self.num as u16;
        let freed_ref = self.latest_freed_data.clone();
        std::thread::spawn(move || {
            let socket = UdpSocket::bind((Self::BASE_ADDRESS, port)).unwrap();
            let mut buf = [0u8; 29];

            while running.load(Ordering::Relaxed) {
                if let Ok((amount, _source)) = socket.recv_from(&mut buf) {
                    if amount == 29 {
                        if let Ok(freed) = FreeD::try_from(&buf) {
                            //println!("{freed:?}");
                            *freed_ref.lock().unwrap() = freed;
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

        self
    }

    #[allow(unused)]
    pub fn set_freed_data(&mut self, freed: FreeD) {
        *self.latest_freed_data.lock().unwrap() = freed;
    }

    pub fn num(&self) -> u8 {
        self.num
    }

    pub fn yaw_pitch_zoom(&self) -> (f32, f32, u32) {
        let data = self.latest_freed_data.lock().unwrap();

        (data.tilt, data.pan, data.zoom)
    }
}