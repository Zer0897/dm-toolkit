use gtk::prelude::EditableSignals;
use gtk::Orientation::Vertical;
use gtk::{ButtonExt, EntryExt, OrientableExt, WidgetExt};
use relm::{connect, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

pub struct Model {
    counter: i32,
}

#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Changed,
}

#[widget]
impl Widget for Counter {
    fn model() -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Decrement => self.model.counter -= 1,
            Increment => self.model.counter += 1,
            Changed => {
                self.model.counter = self
                    .entry
                    .get_text()
                    .map(|v| v.parse().unwrap_or_else(|_| self.model.counter))
                    .unwrap_or_else(|| self.model.counter)
            }
        }
    }

    view! {
        gtk::Box {
            name: "widget",
            orientation: Vertical,
            gtk::Button {
                label: "+",
                name: "inc_button",
                clicked => Increment,
            },
            #[name="entry"]
            gtk::Entry {
                text: &self.model.counter.to_string(),
                alignment: 0.5,
                changed => Changed,
            },
            gtk::Button {
                label: "-",
                name: "dec_button",
                clicked => Decrement,
            },
        }
    }
}
