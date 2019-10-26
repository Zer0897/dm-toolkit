use gdk::{EventMask, ScrollDirection};
use gtk::{
    ButtonExt, ContainerExt, EditableExt, Entry, EntryExt, Inhibit, WidgetExt, WidgetExtManual,
};
use relm::{connect, connect_stream, Relm, Update, Widget};
use relm_derive::Msg;

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
pub enum CounterMsg<T>
where
    T: unit::Unit,
{
    Increment(T),
    Decrement(T),
    Changed(Entry, T),
    Focused(Entry),
    Scrolled(ScrollDirection, T),
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
    type Msg = CounterMsg<T>;

    fn model(_: &Relm<Self>, units: &'static [UnitView<T>]) -> UnitCounterModel<T> {
        UnitCounterModel {
            units,
            count: unit::UnitCounter::new(),
            counters: vec![],
            _unit: std::marker::PhantomData,
        }
    }

    fn update(&mut self, event: CounterMsg<T>) {
        match event {
            CounterMsg::Increment(unit) | CounterMsg::Scrolled(ScrollDirection::Up, unit) => {
                self.model.count.add_units(1, &unit).unwrap_or_default()
            }
            CounterMsg::Decrement(unit) | CounterMsg::Scrolled(ScrollDirection::Down, unit) => {
                self.model.count.sub_units(1, &unit).unwrap_or_default()
            }
            CounterMsg::Changed(entry, unit) => {
                if let Some(text) = entry.get_text() {
                    self.model
                        .count
                        .set_from_string(&text, &unit)
                        .unwrap_or_default();
                }
            }
            CounterMsg::Focused(entry) => {
                entry.select_region(0, -1);
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
                CounterMsg::Changed(entry.clone(), unit.unit)
            );
            connect!(
                relm,
                entry,
                connect_focus_out_event(entry, _),
                return (
                    CounterMsg::Changed(entry.clone(), unit.unit),
                    Inhibit(false)
                )
            );
            connect!(
                relm,
                entry,
                connect_focus_in_event(widget, _),
                return (CounterMsg::Focused(widget.clone()), Inhibit(false))
            );
            connect!(
                relm,
                entry,
                connect_scroll_event(_, scroll),
                return (
                    CounterMsg::Scrolled(scroll.get_direction(), unit.unit),
                    Inhibit(false)
                )
            );
            connect!(
                relm,
                btn_inc,
                connect_clicked(_),
                CounterMsg::Increment(unit.unit)
            );
            connect!(
                relm,
                btn_dec,
                connect_clicked(_),
                CounterMsg::Decrement(unit.unit)
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
