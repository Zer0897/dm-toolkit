use gtk::{BoxExt, LabelExt, WidgetExt};
use itertools::Itertools;
use relm::{Component, Widget};
use relm_derive::{widget, Msg};

use crate::time::UnitTime;
use crate::ui::count::{Counter, CounterMsg};
use crate::ui::edit::EditView;
use crate::ui::text::Markup;
use crate::unit::{Unit, UnitCounter};

type TimeCounter = Counter<UnitTime, ClockDisplay>;

pub struct DisplayModel {
    _counter: Option<Component<TimeCounter>>,
    count: UnitCounter<UnitTime>,
}

#[widget]
impl Widget for ClockDisplay {
    fn model(_counter: Option<Component<TimeCounter>>) -> DisplayModel {
        DisplayModel {
            _counter,
            count: UnitCounter::new(),
        }
    }

    fn init_view(&mut self) {
        self.update_datetime();
    }

    fn update(&mut self, event: CounterMsg<UnitTime>) {
        match event {
            CounterMsg::Change(text, unit) => self
                .model
                .count
                .set_from_text(&text, &unit)
                .unwrap_or_default(),
            CounterMsg::Increment(unit) => self.model.count.add_units(1, &unit).unwrap_or_default(),
            CounterMsg::Decrement(unit) => self.model.count.sub_units(1, &unit).unwrap_or_default(),
            CounterMsg::Add(count, unit) => {
                self.model.count.add_units(count, &unit).unwrap_or_default()
            }
        }
        self.model.count.redistribute().unwrap_or_default();
        self.update_datetime();
    }

    view! {
        gtk::Box {
            halign: gtk::Align::Center,
            hexpand: true,

            #[name="label_date"]
            gtk::Label {
                halign: gtk::Align::Start,
                use_markup: true,
                margin_end: 80,
            },

            #[name="label_time"]
            gtk::Label {
                use_markup: true,
                margin_start: 80,
            },
        }
    }
}

impl ClockDisplay {
    fn update_datetime(&mut self) {
        self.label_date
            .set_markup(&self.date().markup_bold().markup_fontsize(40));
        self.label_time
            .set_markup(&self.time().markup_bold().markup_fontsize(40));
    }

    fn time(&self) -> String {
        let text = [UnitTime::Hour, UnitTime::Minute, UnitTime::Second]
            .iter()
            .map(|unit| self.model.count.get_count(&unit).expect("Unit"))
            .map(|count| format!("{:02}", count))
            .join(":");

        text
    }

    fn date(&self) -> String {
        let weeks = self.model.count.get_count(&UnitTime::Week).expect("Weeks");
        let mut days = self.model.count.get_count(&UnitTime::Day).expect("Days");
        days += weeks * UnitTime::Week.value() / UnitTime::Day.value();
        let months = self
            .model
            .count
            .get_count(&UnitTime::Month)
            .expect("Months");
        let years = self.model.count.get_count(&UnitTime::Year).expect("Years");

        let text = format!("{:02}/{:02}/{:02}", months + 1, days + 1, years);
        text
    }
}

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
        EditView<ClockDisplay, Counter<UnitTime, ClockDisplay>> {
            spacing: 20,
        }
    }
}
