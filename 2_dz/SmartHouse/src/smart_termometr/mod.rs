pub struct SmartTermometr {
    temp: f32,
}

impl SmartTermometr {
    pub fn new() -> Self {
        Self { temp: 27. }
    }

    pub fn temp(&self) -> &f32 {
        &self.temp
    }
}
