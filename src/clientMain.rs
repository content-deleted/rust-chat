extern crate azul;
use azul::{prelude::*, widgets::{label::Label, button::Button, text_input::TextInput, text_input::TextInputState}};
use azul::*;
use std::net::UdpSocket;

use std::{str,io};

mod sharedStructs;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Err: Could not bind socket");

    socket.connect("127.0.0.1:8888").expect("Could not connect to server");

    let app = App::new(DataModel::default(socket), AppConfig::default());
    let mut options = WindowCreateOptions::default();
    options.state.title = String::from("");
    app.run(Window::new(options, Css::native()).unwrap()).unwrap();
    
    /*
    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];

        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");

        socket.send(input.as_bytes())
              .expect("Failed to send to server");
        
        socket.recv_from(&mut buffer)
              .expect("Could not read to buffer");

        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer"));
    } */
}


pub struct DataModel {
    socket: UdpSocket,
    text_input: TextInputState,
    messages: Vec<String>,
    currentUsers: Vec<sharedStructs::User>,
    loggedIn: bool,
}

impl DataModel {
    pub fn default(sok: UdpSocket) -> Self {
        Self {
            socket: sok,
            text_input: TextInputState::new(""),
            messages: (0..20).map(|_num| String::new()).collect(),
            currentUsers: Vec::new(),
            loggedIn: false,
        }
    }
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        let button = Button::with_label("Send").dom()
            .with_callback(On::MouseUp, Callback(send_msg));

        let textinput = TextInput::new()
        // ... bind it to self.text_input - will automatically update
        .bind(info.window, &self.text_input, &self)
        // ... and render it in the UI
        .dom(&self.text_input);
        //.with_callback(azul::dom::On::VirtualKeyUp, Callback(send_msg));


        // List of messages
        let messages = self.messages.iter().enumerate().map(|(_idx, item)| {
            NodeData {
                node_type: NodeType::Label(item.to_string()),
                .. Default::default()
            }
        }).collect::<Dom<Self>>();



        Dom::new(NodeType::Div)
            .with_child(messages
                .with_child(Dom::new(NodeType::Div)
                    .with_child(textinput
                        .with_child(Dom::new(NodeType::Div))
                        .with_child(Dom::new(NodeType::Div)
                            .with_child(button
                                
                            )
                        )
                    )
                )
            )
            
    }
}

fn send_msg (app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> UpdateScreen {
     app_state.data.modify(|state| {
        state.messages.push(state.text_input.text.clone());
        state.messages.remove(0);
        state.text_input.text = String::from("");
        }
    );
    UpdateScreen::Redraw
}

fn attemptLogin(state: &mut DataModel, _app_resources: &mut AppResources) -> (UpdateScreen, TerminateDaemon) {
    (UpdateScreen::Redraw, TerminateDaemon::Continue)
}

fn receiver_daemon(state: &mut DataModel, _app_resources: &mut AppResources) -> (UpdateScreen, TerminateDaemon) {
    let mut buffer = [0u8; 1500];
    state.socket.recv_from(&mut buffer).expect("Could not read to buffer");
    (UpdateScreen::Redraw, TerminateDaemon::Continue)
}


