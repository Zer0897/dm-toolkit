use gtk::prelude::*;
use gtk::Inhibit;
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
        #[name="counter"]
        gtk::Box {
            orientation: Vertical,

            #[name="label"]
            gtk::Label {
                label: "0",
                text: &self.model.counter.to_string(),
            },

            #[name="add_button"]
            gtk::Button {
                clicked => Increment,
                label: "+",
            },

            #[name="remove_button"]
            gtk::Button {
                clicked => Decrement,
                label: "-",
            },
        }
    }
}
