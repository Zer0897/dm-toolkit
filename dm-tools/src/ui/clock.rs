use gtk::{BoxExt, LabelExt};
use itertools::Itertools;
use relm::{Component, Widget};
use relm_derive::{widget, Msg};

use crate::time::UnitTime;
use crate::ui::count::{Counter, CounterMsg, UnitView};
use crate::ui::edit::EditView;
use crate::ui::text::Markup;
use crate::unit::UnitCounter;

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

    fn update(&mut self, event: CounterMsg<UnitTime>) {
        match event {
            CounterMsg::Change(text, unit) => self
                .model
                .count
                .set_from_text(&text, &unit)
                .unwrap_or_default(),
            CounterMsg::Increment(unit) => self.model.count.add_units(1, &unit).unwrap_or_default(),
            CounterMsg::Decrement(unit) => self.model.count.sub_units(1, &unit).unwrap_or_default(),
        }
        self.model.count.redistribute().unwrap_or_default();
        let text = FACES
            .iter()
            .map(|face| self.model.count.get_count(&face.unit).expect("Unit"))
            .map(|count| format!("{:02}", count))
            .join(":");

        self.label.set_markup(&text.markup_title());
    }

    view! {
        #[name="label"]
        gtk::Label {
            use_markup: true,
            markup: &"00:00:00".markup_title()
        }
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
