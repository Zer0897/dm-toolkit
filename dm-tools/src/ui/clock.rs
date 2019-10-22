use gtk::{LabelExt, OrientableExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use crate::ui::counter::Counter;

pub struct Model {}

#[derive(Msg)]
pub enum Msg {}

#[widget]
impl Widget for Clock {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {}
    }

    view! {
        gtk::Box {
            #[name="box_hours"]
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    text: "H"
                },
                #[name="hours"]
                Counter {},
            },
            #[name="box_minutes"]
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    text: "M"
                },
                #[name="minutes"]
                Counter {},
            },
            #[name="box_seconds"]
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    text: "S"
                },
                #[name="seconds"]
                Counter {},
            }
        }
    }
}
