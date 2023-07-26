// pattern command
pub trait Command {
    fn enable(&self) -> &str;
    fn disable(&self) -> &str;
}

pub struct SmartLamp;

impl Command for SmartLamp {
    fn enable(&self) -> &str {
        "the lamp is shining"
    }
    fn disable(&self) -> &str {
        "the lamp is off"
    }
}

pub struct SmartSocket;

impl Command for SmartSocket {
    fn enable(&self) -> &str {
        "socket on 220 volt"
    }
    fn disable(&self) -> &str {
        "socket off "
    }
}

// pattern Srategy

pub trait Formatter {
    fn format(&self, data: &[&str], buf: &mut String);
}

pub struct Report;

impl Report {
    pub fn generate<T: Formatter>(g: T, data: &[&str], s: &mut String) {
        g.format(data, s)
    }
}

pub struct TextTable;

impl Formatter for TextTable {
    fn format(&self, data: &[&str], buf: &mut String) {
        buf.push_str("----------------------------------------");
        buf.push('\n');
        buf.push_str(format!("{0: <20} | {1: <20}", "Number Operation", "Name Operation").as_str());
        for (count, val) in data.iter().enumerate() {
            buf.push_str(format!("\n{0: <20} | {1: <20}", count, val).as_str());
        }
        buf.push('\n');
        buf.push_str("----------------------------------------");
        buf.push('\n');
    }
}

pub struct JsonArray;

impl Formatter for JsonArray {
    fn format(&self, data: &[&str], buf: &mut String) {
        buf.push('[');
        for (count, val) in data.iter().enumerate() {
            buf.push_str(format!("{{ num: {} , val: {} }}", count, val).as_str());
            if count < data.len() {
                buf.push(',');
            }
        }
        buf.push(']');
        buf.push('\n');
    }
}

#[derive(Default)]
pub struct SmartHome {
    commands: Vec<Box<dyn Command>>,
}

impl SmartHome {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }
    pub fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    pub fn enable(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.enable()).collect()
    }

    pub fn disable(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.disable())
            .collect()
    }
}
