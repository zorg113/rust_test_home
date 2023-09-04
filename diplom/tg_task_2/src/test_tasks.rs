use chrono::naive::NaiveDate;
use std::time::SystemTime;
use std::vec;

pub struct TasksData {
    pub name: &'static str,
    //pub start: NaiveDate,
    //pub end: NaiveDate,
}

static TASKS_TEST: &[&'static TasksData] = &[
    &TasksData {
        name: "Создание отчета по разработке кода",
        //start: NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap(),
        //end: NaiveDate::parse_from_str("2023-02-01", "%Y-%m-%d").unwrap(),
    },
    &TasksData {
        name: "Совещание по разработке кода",
        //start: NaiveDate::parse_from_str("2023-01-05", "%Y-%m-%d").unwrap(),
        //end: NaiveDate::parse_from_str("2023-01-06", "%Y-%m-%d").unwrap(),
    },
    &TasksData {
        name: "Коммандировка в Новосибирск",
        //start: NaiveDate::parse_from_str("2023-01-08", "%Y-%m-%d").unwrap(),
        //end: NaiveDate::parse_from_str("2023-01-14", "%Y-%m-%d").unwrap(),
    },
    &TasksData {
        name: "Коммандировка в Нижний Новгород",
        //start: NaiveDate::parse_from_str("2023-01-08", "%Y-%m-%d").unwrap(),
        //end: NaiveDate::parse_from_str("2023-01-14", "%Y-%m-%d").unwrap(),
    },
    &TasksData {
        name: "Создание руководства пользователя кода\n 23.05.08",
        //start: NaiveDate::parse_from_str("2023-01-14", "%Y-%m-%d").unwrap(),
        //end: NaiveDate::parse_from_str("2023-01-18", "%Y-%m-%d").unwrap(),
    },
];

pub fn get_tasks_data(num: usize) -> Option<Vec<&'static TasksData>> {
    TASKS_TEST.chunks(50).nth(num).map(|v| v.to_vec())
}
