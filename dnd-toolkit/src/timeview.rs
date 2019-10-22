use gtk::{LabelExt};
use relm::{Widget};
use relm_derive::{widget, Msg};

pub struct Model {}

#[derive(Msg)]
pub enum Msg {}

#[widget]
impl Widget for TimeView {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {}
    }

    view! {
        gtk::Paned {
            gtk::Label {
                text: "Calender"
            },
            gtk::Label {
                text: "Clock"
            }
        }
    }
}
