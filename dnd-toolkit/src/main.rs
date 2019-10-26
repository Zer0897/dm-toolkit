mod encounter_view;
mod time_view;

use gtk::{Inhibit, NotebookExt, WidgetExt};
use relm::{connect, connect_stream, Widget};
use relm_derive::{widget, Msg};

use encounter_view::EncounterView;
use time_view::TimeView;

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
            gtk::Notebook{
                #[name="tabs"]
                TimeView {
                    child: {
                        tab_label: Some("Time")
                    }
                },
                EncounterView {
                    child: {
                        tab_label: Some("Encounter")
                    }
                },
                gtk::Box {
                    child: {
                        tab_label: Some("Character")
                    }
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
