use gtk::{OrientableExt, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};

use dm_tools::ui::clock::Clock;
use dm_tools::ui::view::Header;

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
        ClockView {},
    }
}

pub struct ClockModel {}

#[derive(Msg)]
pub enum ClockMsg {}

#[widget]
impl Widget for ClockView {
    fn model() -> ClockModel {
        ClockModel {}
    }

    fn update(&mut self, event: ClockMsg) {
        match event {}
    }

    view! {
        gtk::Frame {
            halign: gtk::Align::Center,
            valign: gtk::Align::Center,

            Clock {}
        }
    }
}

pub struct CalendarModel {}

#[derive(Msg)]
pub enum CalendarMsg {}

#[widget]
impl Widget for CalendarView {
    fn model() -> CalendarModel {
        CalendarModel {}
    }

    fn update(&mut self, event: CalendarMsg) {
        match event {}
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            valign: gtk::Align::Center,

            Header("Calendar") {},
            gtk::Calendar {
                valign: gtk::Align::Center,
            },
        }
    }
}
