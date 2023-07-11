use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};

pub struct SmartThermo {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}


#[derive(Serialize,Deserialize)]
pub struct TermoData{
    pub farenheit: f32,
    pub degree_celsius: f32,    
}

#[derive(Clone, Debug)]
pub struct Buffer {
    buff:  Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { buff: vec![] }
    }

    pub fn alloc(size: usize) -> Self {
        let mut in_buf:Vec<u8> = Vec::new();
        in_buf.resize(size, 0);
        Buffer { buff: in_buf }
    }

    pub fn len(self) -> usize {
        self.buff.len()
    }
    pub fn write_message(&mut self, str: &String) {
        let size_msg = str.len().to_be_bytes();
        self.buff.extend(size_msg);
        self.buff.extend(str.as_bytes())
    }

    pub fn read_message(&self) -> String {
        let size =8+  usize::from_be_bytes(self.buff[0..8].try_into().unwrap());
        let s = String::from_utf8_lossy(&self.buff[8..size]).clone();
        s.to_string()
    }

    pub fn to_be_bytes<'a> (&'a  mut self) -> &'a mut [u8]{
        self.buff.as_mut_slice()
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
            let mut buff_prev: [u8; 8]= [0; 8];
            match socket.peek(&mut buff_prev) {
                Ok(_) => (),
                Err(e) => println!("peek function failed: {e:?}"),
            }
            let number_of_bytes : usize = usize::from_be_bytes(buff_prev);
            let mut buff = Buffer::alloc(number_of_bytes+8);
            if let Err(err) = socket.recv_from(buff.to_be_bytes()) {
                println!("can't receive datagram: {err}");
            }
            let val =buff.read_message();
            temperature_clone.set(45);
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
