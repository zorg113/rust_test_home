use chrono::naive::NaiveDate;
use std::time::SystemTime;
use std::vec;

pub struct TasksData {
    pub name: &'static str,
    pub id:  i32,
    pub start: &'static str,
    pub end: &'static str,
}

static TASKS_TEST: &[&'static TasksData] = &[
    &TasksData {
        name: "Создание отчета по разработке кода",
        id: 1,
        start: "2023-01-01",
        end: "2023-02-01",
    },
    &TasksData {
        name: "Совещание по разработке кода",
        id: 2,
        start: "2023-01-05",
        end: "2023-01-06",
    },
    &TasksData {
        name: "Коммандировка в Новосибирск",
        id: 3,
        start: "2023-01-08",
        end: "2023-01-14",
    },
    &TasksData {
        name: "Коммандировка в Нижний Новгород",
        id: 4,
        start: "2023-01-08",
        end: "2023-01-14",
    },
    &TasksData {
        name: "Создание руководства пользователя кода",
        id: 5,
        start: "2023-01-14",
        end: "2023-01-18",
    },
];

pub fn get_tasks_data(num: usize) -> Option<Vec<&'static TasksData>> {
    TASKS_TEST.chunks(50).nth(num).map(|v| v.to_vec())
}

pub fn get_task_data_by_id(id: i32) ->Option<&'static TasksData>{
    let data = TASKS_TEST.to_vec();
    for i in data {
        if i.id == id {
            return Some(i)
        } 
    }
    None
}
