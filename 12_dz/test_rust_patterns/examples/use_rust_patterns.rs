use test_patterns::patterns::{JsonArray, Report, SmartHome, SmartLamp, SmartSocket, TextTable};

fn main() {
    let mut smart_home = SmartHome::new();
    let cmd = Box::new(SmartSocket);
    smart_home.add_command(cmd);
    let cmd = Box::new(SmartLamp);
    smart_home.add_command(cmd);
    let mut report_on = String::new();
    //
    let result_enable = &smart_home.enable();
    Report::generate(TextTable, result_enable, &mut report_on);
    print!("{}", report_on);
    report_on.clear();
    Report::generate(JsonArray, result_enable, &mut report_on);
    print!("{}", report_on);
    let mut report_off = String::new();
    //
    println!("\n ---- Revers operation ----\n");
    let result_disable = &smart_home.disable();
    Report::generate(TextTable, result_disable, &mut report_off);
    print!("{}", report_off);
    report_off.clear();
    Report::generate(JsonArray, result_disable, &mut report_off);
    print!("{}", report_off);
}
