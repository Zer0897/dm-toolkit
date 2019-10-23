use gtk::HeaderBarExt;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct Model {
    title: &'static str,
}

#[derive(Msg)]
pub enum Msg {}

#[widget]
impl Widget for Header {
    fn model(title: &'static str) -> Model {
        Model { title }
    }

    fn update(&mut self, event: Msg) {
        match event {}
    }

    view! {
        gtk::HeaderBar {
            title: Some(self.model.title)
        }
    }
}
