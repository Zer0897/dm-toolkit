use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Inhibit, OrientableExt, WidgetExt};
use relm::{connect, connect_stream, Widget};
use relm_derive::{widget, Msg};

use dm_tools::ui::counter::Counter;

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            Counter,
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
