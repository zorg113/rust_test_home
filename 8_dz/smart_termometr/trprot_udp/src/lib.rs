use std::{
    error::Error,
    mem::size_of,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct SmartThermo {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}

#[derive(Clone, Debug)]
pub struct Buffer {
    buff: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { buff: vec![] }
    }

    pub fn len(self) -> usize {
        self.buff.len()
    }
    pub fn write_message(&mut self, str: &String) {
        self.buff.extend(str.as_bytes())
    }

    pub fn read_message(&self) -> String {
        let s = String::from_utf8_lossy(self.buff.as_slice());
        s.to_string().clone()
    }
}

impl SmartThermo {
    pub fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address)?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;

        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Temperature::default());

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        thread::spawn(move || loop {
            if finished_clone.load(Ordering::SeqCst) {
                return;
            }
            // let (number_of_bytes, src_addr) = socket.peek_from(&mut buf)
            //                                         .expect("Didn't receive data");
            let mut buf = [0; 4];
            if let Err(err) = socket.recv_from(&mut buf) {
                println!("can't receive datagram: {err}");
            }

            let val = f32::from_be_bytes(buf);
            temperature_clone.set(val);
        });

        Ok(Self {
            temperature,
            finished,
        })
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature.get()
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

#[derive(Default)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub fn get(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, val: f32) {
        *self.0.lock().unwrap() = val
    }
}
