use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::{Mutex, MutexGuard},
    time,
};

use serde::{Deserialize, Serialize};

pub struct SmartThermo {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}

#[derive(Serialize, Deserialize)]
pub struct TermoData {
    pub farenheit: f32,
    pub degree_celsius: f32,
}

#[derive(Default, Clone, Debug)]
pub struct Buffer {
    buff: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer { buff: vec![] }
    }

    pub fn alloc(size: usize) -> Self {
        let mut in_buf: Vec<u8> = Vec::new();
        in_buf.resize(size, 0);
        Buffer { buff: in_buf }
    }

    pub fn write_message(&mut self, str: &String) {
        let size_msg = str.len().to_be_bytes();
        self.buff.extend(size_msg);
        self.buff.extend(str.as_bytes())
    }

    pub fn read_message(&self) -> String {
        let size = 8 + usize::from_be_bytes(self.buff[0..8].try_into().unwrap());
        let s = String::from_utf8_lossy(&self.buff[8..size]).clone();
        s.to_string()
    }

    pub fn to_be_bytes(&mut self) -> &mut [u8] {
        self.buff.as_mut_slice()
    }
}

impl SmartThermo {
    pub async fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address).await?;
        let timeout = Duration::from_secs(1);

        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Temperature::default());

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();
        tokio::spawn(async move {
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }
                let mut buff = Buffer::alloc(1024);
                if let Err(err) = time::timeout(timeout, socket.recv_from(buff.to_be_bytes())).await
                {
                    println!("can't receive datagram: {err}");
                }
                let val = buff.read_message();
                temperature_clone.set(val).await;
            }
        });

        Ok(Self {
            temperature,
            finished,
        })
    }

    pub async fn get_temperature(&self) -> String {
        self.temperature.get().await.to_string()
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

#[derive(Default)]
struct Temperature {
    s: Arc<Mutex<String>>,
}

impl Temperature {
    pub async fn get(&self) -> MutexGuard<String> {
        self.s.lock().await
    }

    pub async fn set(&self, val: String) {
        *self.s.lock().await = val
    }
}
