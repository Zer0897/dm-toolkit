use gtk::prelude::EditableSignals;
use gtk::Orientation::Vertical;
use gtk::{ButtonExt, ContainerExt, EntryExt, OrientableExt, WidgetExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::{widget, Msg};

use crate::unit::Unit;

use self::Msg::*;

pub struct Model {
    counter: i32,
}

#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Changed,
}

#[widget]
impl Widget for Counter {
    fn model() -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Decrement => self.model.counter -= 1,
            Increment => self.model.counter += 1,
            Changed => {
                self.model.counter = self
                    .entry
                    .get_text()
                    .map(|v| v.parse().unwrap_or_else(|_| self.model.counter))
                    .unwrap_or_else(|| self.model.counter)
            }
        }
    }

    view! {
        gtk::Box {
            name: "widget",
            orientation: Vertical,
            gtk::Button {
                label: "+",
                name: "inc_button",
                clicked => Increment,
            },
            #[name="entry"]
            gtk::Entry {
                text: &self.model.counter.to_string(),
                alignment: 0.5,
                changed => Changed,
            },
            gtk::Button {
                label: "-",
                name: "dec_button",
                clicked => Decrement,
            },
        }
    }
}

struct CountBase<T>
where
    T: Unit,
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
    T: Unit,
{
    Increment(T),
    Decrement(T),
    Changed(T),
}

pub struct UnitCounterModel<T>
where
    T: Unit,
{
    units: &'static [T],
    counters: Vec<CountBase<T>>,
    count: usize,
    _unit: std::marker::PhantomData<T>,
}

pub struct UnitCounter<T>
where
    T: Unit,
{
    _unit: std::marker::PhantomData<T>,
    window: gtk::Box,
    model: UnitCounterModel<T>,
}

impl<T> Update for UnitCounter<T>
where
    T: Unit,
{
    type Model = UnitCounterModel<T>;
    type ModelParam = (&'static [T]);
    type Msg = CounterMsg<T>;

    fn model(_: &Relm<Self>, units: &'static [T]) -> UnitCounterModel<T> {
        UnitCounterModel {
            units,
            count: 0,
            counters: vec![],
            _unit: std::marker::PhantomData,
        }
    }

    fn update(&mut self, event: CounterMsg<T>) {
        match event {
            CounterMsg::Increment(u) => self.model.count += u.value(),
            CounterMsg::Decrement(u) => self.model.count -= u.value(),
            CounterMsg::Changed(u) => {
                let mut current_count = T::distribute(self.model.count);
                let changed = self.model.counters.iter().find(|c| c.unit == u).map(|c| {
                    c.entry
                        .get_text()
                        .map(|v| {
                            v.parse::<usize>()
                                .unwrap_or_else(|_| *current_count.get(&c.unit).unwrap_or(&0))
                        })
                        .unwrap_or_else(|| *current_count.get(&c.unit).unwrap_or(&0))
                });

                if let Some(changed) = changed {
                    current_count
                        .get_mut(&u)
                        .map(|v| *v = changed)
                        .unwrap_or_else(|| {
                            current_count.insert(u, changed);
                        });
                }
                self.model.count = current_count.iter().map(|(k, v)| k.value() * v).sum();
            }
        }
        let new_count = T::distribute(self.model.count);
        for counter in self.model.counters.iter_mut() {
            let unit_count = *new_count.get(&counter.unit).unwrap_or(&0);
            counter.entry.set_text(&unit_count.to_string());
        }
    }
}

impl<T> Widget for UnitCounter<T>
where
    T: Unit,
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
            entry.set_text(&model.count.to_string());

            let btn_inc = gtk::Button::new();
            btn_inc.set_label("+");

            let btn_dec = gtk::Button::new();
            btn_dec.set_label("-");

            connect!(relm, entry, connect_activate(_), CounterMsg::Changed(*unit));
            connect!(
                relm,
                btn_inc,
                connect_clicked(_),
                CounterMsg::Increment(*unit)
            );
            connect!(
                relm,
                btn_dec,
                connect_clicked(_),
                CounterMsg::Decrement(*unit)
            );

            counter.add(&btn_inc);
            counter.add(&entry);
            counter.add(&btn_dec);

            counter.show_all();

            model.counters.push(CountBase {
                entry,
                btn_inc,
                btn_dec,
                unit: *unit,
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
