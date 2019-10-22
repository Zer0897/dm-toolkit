use gtk::{BoxExt, OrientableExt, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use dm_tools::ui::clock::Clock;

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
            orientation: gtk::Orientation::Vertical,

            gtk::Calendar {
                valign: gtk::Align::Center,
            },
            gtk::Box {
                halign: gtk::Align::Center,
                valign: gtk::Align::Center,

                Clock {
                    spacing: 8,
                    halign: gtk::Align::Center,
                    valign: gtk::Align::Center,
                }
            }
        }
    }
}
