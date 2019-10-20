mod counter;

use gtk::prelude::*;
use gtk::Inhibit;
use gtk::Orientation::Vertical;
use relm::{connect, connect_stream, init, Component, Widget};
use relm_derive::{widget, Msg};

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {
    counter: Component<counter::Counter>,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        let counter = init::<counter::Counter>(()).expect("Counter");

        Model { counter }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            titlebar: Some(self.model.counter.widget()),

            #[name="app"]
            gtk::Box {
                orientation: Vertical
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Window::run");
}
