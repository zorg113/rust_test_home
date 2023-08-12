use iced::alignment;
use iced::executor;
use iced::widget::container;
use iced::widget::{column, row, text, Button};
use iced::window;
use iced::Color;
use iced::{Application, Command, Element, Length, Settings, Theme};
use serde::{Deserialize, Serialize};
use std::error::Error;
use trprot::client_trprot::TrprotClient;

#[derive(Serialize, Deserialize)]
enum Request {
    GetStatus,
    SetStatus,
}

#[derive(Serialize, Deserialize)]
enum Status {
    On,
    Off,
    None,
}

#[derive(Serialize, Deserialize)]
enum TcpMessage {
    Request {
        id: Request,
        value: Status,
    },
    Response {
        id: Request,
        value: Status,
        power: f32,
    },
}

struct SmartSocketClient {
    smart_socket_status: String,
    color_status: Color,
}
pub fn main() -> iced::Result {
    SmartSocketClient::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum GuiMessage {
    GetStatusSmartSocket,
    DisableSmartSocket,
    EnableSmartSocket,
    Exit,
}

impl Default for SmartSocketClient {
    fn default() -> Self {
        SmartSocketClient {
            smart_socket_status: String::from("Unknown"),
            color_status: Color {
                r: 255.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        }
    }
}

impl SmartSocketClient {
    fn update(&mut self) {
        let mesg_get_status = TcpMessage::Request {
            id: Request::GetStatus,
            value: Status::None,
        };
        (self.smart_socket_status, self.color_status) = match send_command(mesg_get_status) {
            Ok(_s) => (
                _s,
                Color {
                    r: 0.1,
                    g: 0.68,
                    b: 0.1,
                    a: 1.,
                },
            ),
            Err(_err) => (
                _err.to_string(),
                Color {
                    r: 1.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
            ),
        };
    }

    fn enable(&mut self) {
        let mesg_on = TcpMessage::Request {
            id: Request::SetStatus,
            value: Status::On,
        };
        (self.smart_socket_status, self.color_status) = match send_command(mesg_on) {
            Ok(_s) => (
                _s,
                Color {
                    r: 0.1,
                    g: 0.68,
                    b: 0.1,
                    a: 1.,
                },
            ),
            Err(_err) => (
                _err.to_string(),
                Color {
                    r: 1.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
            ),
        };
    }

    fn disable(&mut self) {
        let mesg_off = TcpMessage::Request {
            id: Request::SetStatus,
            value: Status::Off,
        };
        (self.smart_socket_status, self.color_status) = match send_command(mesg_off) {
            Ok(_s) => (
                _s,
                Color {
                    r: 0.1,
                    g: 0.68,
                    b: 0.1,
                    a: 1.,
                },
            ),
            Err(_err) => (
                _err.to_string(),
                Color {
                    r: 1.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
            ),
        };
    }
}

impl Application for SmartSocketClient {
    type Message = GuiMessage;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("SmartSocketClient")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::Exit => window::close(),
            Self::Message::GetStatusSmartSocket => {
                self.update();
                Command::none()
            }
            Self::Message::EnableSmartSocket => {
                self.enable();
                Command::none()
            }
            Self::Message::DisableSmartSocket => {
                self.disable();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let exit_btn =
            Button::new(text("Exit").horizontal_alignment(alignment::Horizontal::Center))
                .padding(6)
                .height(40)
                .on_press(Self::Message::Exit);
        let get_status_btn =
            Button::new(text("GetStatus").horizontal_alignment(alignment::Horizontal::Center))
                .padding(6)
                .height(40)
                .on_press(Self::Message::GetStatusSmartSocket);
        let enable_ssocket_btn =
            Button::new(text("Enable").horizontal_alignment(alignment::Horizontal::Center))
                .padding(6)
                .height(40)
                .on_press(Self::Message::EnableSmartSocket);
        let disable_ssocket_btn =
            Button::new(text("Disable").horizontal_alignment(alignment::Horizontal::Center))
                .padding(6)
                .height(40)
                .on_press(Self::Message::DisableSmartSocket);
        let lable_connect_status = column![
            "SmartSocketStatus",
            text(&self.smart_socket_status).style(self.color_status)
        ]
        .spacing(5);
        let content = row![
            exit_btn,
            get_status_btn,
            enable_ssocket_btn,
            disable_ssocket_btn
        ]
        .spacing(5);
        let base = column![content, lable_connect_status].spacing(20);
        container(base)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn send_command(msg: TcpMessage) -> Result<String, Box<dyn Error>> {
    let mut client = TrprotClient::connect("127.0.0.1:55331")?;
    let mesg = serde_json::to_string(&msg).unwrap();
    let response = client.send_request(mesg)?;
    Ok(response)
}
