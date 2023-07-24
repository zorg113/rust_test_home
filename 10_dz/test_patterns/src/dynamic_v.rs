pub struct Socket {
    value: String,
}

impl Socket {
    pub fn new() -> Self {
        Socket {
            value: "Off".to_string(),
        }
    }
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Termometr {
    value: String,
}

impl Termometr {
    pub fn new() -> Self {
        Termometr {
            value: "Off".to_string(),
        }
    }
}

impl Default for Termometr {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SmartObject {
    fn get_report(&self) -> String;
    fn accept(&mut self, v: &impl Visitor);
}

impl SmartObject for Socket {
    fn get_report(&self) -> String {
        format!("{0} SmartSocket 220V", self.value)
    }
    fn accept(&mut self, v: &impl Visitor) {
        v.visit_socket(self);
    }
}

impl SmartObject for Termometr {
    fn get_report(&self) -> String {
        format!("{0} SmartTermometer Temp 27 C", self.value)
    }
    fn accept(&mut self, v: &impl Visitor) {
        v.visit_termometr(self);
    }
}

pub trait Visitor {
    fn visit_socket(&self, status: &mut Socket);
    fn visit_termometr(&self, status: &mut Termometr);
}

pub struct PrintStatus;

impl Visitor for PrintStatus {
    fn visit_termometr(&self, status: &mut Termometr) {
        println!("Status is termoment {0}", status.value);
    }
    fn visit_socket(&self, status: &mut Socket) {
        println!("Status is socket {0}", status.value);
    }
}

pub struct ChangeStatus;

impl Visitor for ChangeStatus {
    fn visit_termometr(&self, status: &mut Termometr) {
        status.value = "Off".to_string();
    }
    fn visit_socket(&self, status: &mut Socket) {
        status.value = "On".to_string();
    }
}
