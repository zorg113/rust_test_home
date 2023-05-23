/// Умная розетка
pub struct SmartSocket {
    /// Текстовое описание розетки
    description: String,
    /// состояние вкл/выкл
    on_off: bool,
    /// потребляемая мощность
    power: f32,
}
///
impl SmartSocket {
    pub fn new() -> Self {
        Self {
            description: String::from("Text"),
            on_off: false,
            power: 0.0,
        }
    }

    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub fn on_off(&self) -> bool {
        self.on_off
    }

    pub fn power(&self) -> &f32 {
        &self.power
    }
}
