use azul::{prelude::*, widgets::{label::Label, button::Button, text_input::TextInput, text_input::TextInputState}};

struct DataModel {
    counter: usize,
    text_input: TextInputState,
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        //let label = Label::new(format!("{}", self.text_input )).dom();
        let button = Button::with_label("Send").dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        let textinput = TextInput::new()
        // ... bind it to self.text_input - will automatically update
        .bind(info.window, &self.text_input, &self)
        // ... and render it in the UI
        .dom(&self.text_input)
        .with_callback(azul::dom::On::VirtualKeyUp, Callback(send_msg));

        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(button)
            .with_child(textinput)
    }
}

// View updates Model
fn update_counter(app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    UpdateScreen::Redraw
}

fn send_msg (app_state: &mut AppState<DataModel>, _event: WindowEvent<DataModel>) -> UpdateScreen {
    UpdateScreen::Redraw
}

pub fn startGUI () {
    let app = App::new(DataModel { counter: 0, text_input: TextInputState::new("") }, AppConfig::default());
    app.run(Window::new(WindowCreateOptions::default(), Css::native()).unwrap()).unwrap();
}