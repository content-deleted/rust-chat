
// gui information

use relm::{Relm, Update, Widget};
use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};

pub struct Model {

}

#[derive(Msg)]
pub enum Msg {
    // …
    Quit,
}

pub struct Win {
    model: Model,
    window: Window,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
        }
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            window: window,
        }
    }
}