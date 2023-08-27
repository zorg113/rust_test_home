use chrono::DateTime;
use std::vec::Vec;

pub struct SubTask {
    name: String,
    start: DateTime,
    end: DateTime,
    id_parenv: u32,
}

pub struct Task {
    name: String,
    start: DateTime,
    end: DateTime,
    sub_task: Vec<SubTask>,
}
