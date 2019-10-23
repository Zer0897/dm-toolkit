use relm::Widget;
use relm_derive::{widget, Msg};

use crate::time::UnitTime;
use crate::ui::counter::UnitCounter;

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
            UnitCounter<UnitTime>(&[UnitTime::Hour, UnitTime::Minute, UnitTime::Second]) {}
        }
    }
}
