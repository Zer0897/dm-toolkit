use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {}

#[derive(Msg)]
pub enum Msg {}

#[widget]
impl Widget for EncounterView {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {}
    }

    view! {
        gtk::Box {
        }
    }
}
