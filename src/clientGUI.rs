use azul::{prelude::*, widgets::{label::Label, button::Button, text_input::TextInput, text_input::TextInputState}};
use azul::*;
use std::net::UdpSocket;

pub struct DataModel {
    socket: UdpSocket,
    text_input: TextInputState,
    messages: Vec<String>,
    currentUsers: Vec<User>,
}

struct User {
    username: String,
    password: String,
}

impl DataModel {
    pub fn default(sok: UdpSocket) -> Self {
        Self {
            socket: sok,
            text_input: TextInputState::new(""),
            messages: (0..20).map(|_num| String::new()).collect(),
            currentUsers: Vec::new()
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

fn receiver_daemon(state: &mut DataModel, _app_resources: &mut AppResources) -> (UpdateScreen, TerminateDaemon) {
    let mut buffer = [0u8; 1500];
    //socket.recv_from(&mut buffer).expect("Could not read to buffer");
    (UpdateScreen::Redraw, TerminateDaemon::Continue)
}


