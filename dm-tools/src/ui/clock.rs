use relm::Widget;
use relm_derive::{widget, Msg};

use crate::time::UnitTime;
use crate::ui::counter::{UnitCounter, UnitView};

const FACES: [UnitView<UnitTime>; 3] = [
    UnitView {
        name: Some("Hours"),
        unit: UnitTime::Hour,
    },
    UnitView {
        name: Some("Minutes"),
        unit: UnitTime::Minute,
    },
    UnitView {
        name: Some("Seconds"),
        unit: UnitTime::Second,
    },
];

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
            UnitCounter<UnitTime>(&FACES) {}
        }
    }
}
