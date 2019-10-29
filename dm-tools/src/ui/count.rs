use gdk::enums::key;
use gdk::{EventMask, ScrollDirection};
use gtk::{
    ButtonExt, ContainerExt, EditableExt, EntryExt, Inhibit, LabelExt, WidgetExt, WidgetExtManual,
};
use relm::{connect, connect_stream, Component, Relm, Update, Widget};
use relm_derive::{widget, Msg};

use crate::ui::edit::EditView;
use crate::unit;

struct CountBase<T>
where
    T: unit::Unit,
{
    entry: gtk::Entry,
    #[allow(dead_code)]
    btn_inc: gtk::Button,
    #[allow(dead_code)]
    btn_dec: gtk::Button,
    unit: T,
}

#[derive(Msg, Debug)]
pub enum UnitCounterMsg<T>
where
    T: unit::Unit,
{
    Increment(T),
    Decrement(T),
    Change(gtk::Entry, T),
    Focus(gtk::Entry),
    Scroll(ScrollDirection, T),
    KeyPress(key::Key, T),
}

pub struct UnitCounterModel<T>
where
    T: unit::Unit,
{
    units: &'static [UnitView<T>],
    counters: Vec<CountBase<T>>,
    count: unit::UnitCounter<T>,
    _unit: std::marker::PhantomData<T>,
}

pub struct UnitCounter<T>
where
    T: unit::Unit,
{
    _unit: std::marker::PhantomData<T>,
    window: gtk::Box,
    model: UnitCounterModel<T>,
}

impl<T> Update for UnitCounter<T>
where
    T: unit::Unit,
{
    type Model = UnitCounterModel<T>;
    type ModelParam = (&'static [UnitView<T>]);
    type Msg = UnitCounterMsg<T>;

    fn model(_: &Relm<Self>, units: &'static [UnitView<T>]) -> UnitCounterModel<T> {
        UnitCounterModel {
            units,
            count: unit::UnitCounter::new(),
            counters: vec![],
            _unit: std::marker::PhantomData,
        }
    }

    fn update(&mut self, event: UnitCounterMsg<T>) {
        match event {
            UnitCounterMsg::Increment(unit) | UnitCounterMsg::Scroll(ScrollDirection::Up, unit) => {
                self.model.count.add_units(1, &unit).unwrap_or_default()
            }
            UnitCounterMsg::Decrement(unit)
            | UnitCounterMsg::Scroll(ScrollDirection::Down, unit) => {
                self.model.count.sub_units(1, &unit).unwrap_or_default()
            }
            UnitCounterMsg::Change(entry, unit) => {
                if let Some(text) = entry.get_text() {
                    self.model
                        .count
                        .set_from_string(&text, &unit)
                        .unwrap_or_default();
                }
            }
            UnitCounterMsg::Focus(entry) => {
                entry.select_region(0, -1);
            }
            UnitCounterMsg::KeyPress(key::Escape, unit) => {
                self.model
                    .counters
                    .iter()
                    .find(|c| c.unit == unit)
                    .map(|c| {
                        c.entry
                            .set_text(&self.model.count.get_count(&unit).unwrap().to_string())
                    });
            }
            _ => {}
        };
        self.model.count.redistribute().unwrap_or_default();
        for counter in self.model.counters.iter_mut() {
            if let Ok(count) = self.model.count.get_count(&counter.unit) {
                counter.entry.set_text(&count.to_string());
            }
        }
    }
}

impl<T> Widget for UnitCounter<T>
where
    T: unit::Unit,
{
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let mut model = model;

        for unit in model.units.iter() {
            let counter = gtk::Box::new(gtk::Orientation::Vertical, 1);

            let entry = gtk::Entry::new();
            entry.set_text(
                &model
                    .count
                    .get_count(&unit.unit)
                    .map(|v| v.to_string())
                    .expect("Unit"),
            );
            entry.set_input_purpose(gtk::InputPurpose::Number);
            entry.add_events(EventMask::SCROLL_MASK);

            let btn_inc = gtk::Button::new();
            btn_inc.set_label("+");

            let btn_dec = gtk::Button::new();
            btn_dec.set_label("-");

            connect!(
                relm,
                entry,
                connect_activate(entry),
                UnitCounterMsg::Change(entry.clone(), unit.unit)
            );
            connect!(
                relm,
                entry,
                connect_focus_out_event(entry, _),
                return (
                    UnitCounterMsg::Change(entry.clone(), unit.unit),
                    Inhibit(false)
                )
            );
            connect!(
                relm,
                entry,
                connect_focus_in_event(widget, _),
                return (UnitCounterMsg::Focus(widget.clone()), Inhibit(false))
            );
            connect!(
                relm,
                entry,
                connect_scroll_event(_, scroll),
                return (
                    UnitCounterMsg::Scroll(scroll.get_direction(), unit.unit),
                    Inhibit(false)
                )
            );
            connect!(
                relm,
                counter,
                connect_key_press_event(_, event),
                return (
                    UnitCounterMsg::KeyPress(event.get_keyval(), unit.unit),
                    Inhibit(false)
                )
            );
            connect!(
                relm,
                btn_inc,
                connect_clicked(_),
                UnitCounterMsg::Increment(unit.unit)
            );
            connect!(
                relm,
                btn_dec,
                connect_clicked(_),
                UnitCounterMsg::Decrement(unit.unit)
            );

            if unit.name.is_some() {
                counter.add(&gtk::Label::new(unit.name));
            };
            counter.add(&btn_inc);
            counter.add(&entry);
            counter.add(&btn_dec);

            counter.show_all();

            model.counters.push(CountBase {
                entry,
                btn_inc,
                btn_dec,
                unit: unit.unit,
            });

            window.add(&counter);
        }
        window.show_all();

        Self {
            model,
            window,
            _unit: std::marker::PhantomData,
        }
    }
}

pub struct UnitView<T> {
    pub unit: T,
    pub name: Option<&'static str>,
}

pub struct CounterModel {}

#[derive(Msg)]
pub enum CounterMsg {
    Increment,
    Decrement,
    Change(String),
}

#[widget]
impl Widget for CounterLabel {
    fn model() -> CounterModel {
        CounterModel {}
    }

    fn update(&mut self, event: CounterMsg) {
        match event {
            CounterMsg::Change(text) => self.label.set_text(&text),
            _ => {}
        }
    }

    view! {
        #[name="label"]
        gtk::Label {
            text: "Foo"
        }
    }
}

pub struct CounterEditModel {
    view: Component<CounterLabel>,
}

#[widget]
impl Widget for CounterEdit {
    fn model(view: Component<CounterLabel>) -> CounterEditModel {
        CounterEditModel { view }
    }

    fn update(&mut self, event: CounterMsg) {
        match event {
            event => self.model.view.stream().emit(event),
        }
    }

    view! {
        gtk::Box {
            #[name="entry"]
            gtk::Entry {
                activate(entry) => CounterMsg::Change(entry.get_text().expect("Entry text").to_string()),
            },
            gtk::Button {
                label: "+",
                clicked => CounterMsg::Increment,
            },
            gtk::Button {
                label: "-",
                clicked => CounterMsg::Decrement,
            },
        }
    }
}

#[widget]
impl Widget for Counter {
    fn model() -> CounterModel {
        CounterModel {}
    }

    fn update(&mut self, event: CounterMsg) {
        match event {
            _ => {}
        }
    }

    view! {
        EditView<CounterLabel, CounterEdit> {}
    }
}
