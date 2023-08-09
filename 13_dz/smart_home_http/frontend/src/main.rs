use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    devices: HashSet<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    data: SmartHouse,
}

#[derive(Clone)]
struct JsRoom {
    name: String,
    device: String,
}

#[function_component]
fn App() -> Html {
    let forecast = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let retry = {
        let forecast = forecast.clone();
        let error = error.clone();
        Callback::from(move |_: MouseEvent| {
            let forecast = forecast.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let forecast_endpoint = "http://localhost:8000/api/smart_home/report";
                let fetched_forecast = Request::get(&forecast_endpoint).send().await;
                match fetched_forecast {
                    Ok(response) => {
                        let json: Result<Forecast, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                forecast.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    let get_rooms = { Callback::from(move |_: MouseEvent| {}) };
    let get_devices = { Callback::from(move |_: MouseEvent| {}) };
    match (*forecast).as_ref() {
        Some(f) => {
            let mut vec_room: Vec<JsRoom> = vec![];
            for (name_r, dev) in &f.data.rooms {
                let mut js_room = JsRoom {
                    name: name_r.clone(),
                    device: "".to_string(),
                };
                for d in &dev.devices {
                    js_room.device = d.clone();
                    vec_room.push(js_room.clone());
                }
            }
            html! {
                  <table class="table">
                  <tr class="table">
                    <th class="table" >{"Room"}</th>
                    <th class="table">{"Device"}</th>
                  </tr>
                  {
                    vec_room.into_iter().map(|room| {
                        html!{
                            <tr class="table">
                            <td class="table">{ format!("{}",room.name) }</td>
                            <td class="table">{ format!("{}",room.device) }</td>
                            </tr>
                        }
                    }).collect::<Html>()
                  }

                </table>
            }
        }
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry}>{"retry"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        {"No data yet"}
                        <table class="table">
                           <tr>
                                <td>
                                    <button onclick={retry}>{"Call Report"}</button>
                                </td>
                                <td>
                                    <button onclick={get_rooms}>{"Call Show Rooms"}</button>
                                </td>
                                <td>
                                    <button onclick={get_devices}>{"Call Show Device In Room"}</button>
                                </td>
                           </tr>
                        </table>
                    </>
                }
            }
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
