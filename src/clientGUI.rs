use azul::{prelude::*, widgets::{label::Label, button::Button, text_input::TextInput, text_input::TextInputState}};
use azul::*;
struct DataModel {
    counter: usize,
    text_input: TextInputState,
    messages: Vec<String>
}

impl Default for DataModel {
    fn default() -> Self {
        Self {
            text_input: TextInputState::new(""),
            counter: 0,
            messages: (0..20).map(|_num| String::new()).collect()
        }
    }
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        //let label = Label::new(format!("{}", self.counter)).dom();
        //let label = Label::new(format!("{}", self.text_input )).dom();
        let button = Button::with_label("Send").dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        let textinput = TextInput::new()
        // ... bind it to self.text_input - will automatically update
        .bind(info.window, &self.text_input, &self)
        // ... and render it in the UI
        .dom(&self.text_input);
        //.with_callback(azul::dom::On::VirtualKeyUp, Callback(send_msg));


        // List of messages
        let messages = self.messages.iter().enumerate().map(|(idx, item)| {
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

// View updates Model
fn update_counter(app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> UpdateScreen {
    app_state.data.modify(|state| {
        state.counter += 1;
        state.messages.push(state.text_input.text.clone());
        state.messages.remove(0);
        state.text_input.text = String::from("");
        }
    );
    UpdateScreen::Redraw
}

fn send_msg (app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> UpdateScreen {
    UpdateScreen::Redraw
}

pub fn startGUI () {
    let app = App::new(DataModel::default(), AppConfig::default());
    let mut options = WindowCreateOptions::default();
    options.state.title = String::from("test");
    app.run(Window::new(options, Css::native()).unwrap()).unwrap();
}


