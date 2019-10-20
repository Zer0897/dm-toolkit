use gtk::{
    ButtonExt,
    Inhibit,
    LabelExt,
    OrientableExt,
    WidgetExt,
};

use gtk::Orientation::Vertical;
use relm::{connect, connect_stream, init, Component, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    counter: i64,
}

#[widget]
impl Widget for Counter {
    fn model() -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Increment => self.model.counter += 1,
            Decrement => self.model.counter -= 1,
        }

    }

    view! {
        gtk::Box {
            name: "counter",
            orientation: Vertical,

            gtk::Label {
                name: "label",
                label: "0",
                text: &self.model.counter.to_string(),
            },

            gtk::Button {
                name: "add_button",
                clicked => Increment,
                label: "+",
            },

            gtk::Button {
                name: "remove_button",
                clicked => Decrement,
                label: "-",
            },
        }
    }
}
