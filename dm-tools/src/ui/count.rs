use gdk::enums::key;
use gdk::{EventMask, ScrollDirection};
use gtk::{BoxExt, ButtonExt, EntryExt, Inhibit, OrientableExt, WidgetExt, WidgetExtManual};
use relm::{connect, connect_stream, Component, ContainerWidget, EventStream, Relm, Widget};
use relm_derive::{widget, Msg};

use crate::ui::text::TextEntry;
use crate::unit::Unit;

#[derive(Msg)]
pub enum CounterDisplayMsg<T> {
    Count(i64, T),
}

#[derive(Msg)]
pub enum CounterMsg<T: Unit> {
    Change(String, T),
    Add(i64, T),
    Increment(T),
    Decrement(T),
}

pub struct CounterModel<T, E>
where
    T: Unit,
    E: Widget<Msg = CounterMsg<T>>,
{
    display: Option<Component<E>>,
    counters: Vec<Component<CounterEdit<T>>>,
}

#[widget]
impl<T, E> Widget for Counter<T, E>
where
    T: Unit,
    E: Widget<Msg = CounterMsg<T>>,
{
    fn model(display: Option<Component<E>>) -> CounterModel<T, E> {
        CounterModel {
            display,
            counters: vec![],
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        // XXX Requires variants to be sorted
        for unit in T::variants().iter().rev() {
            let widget = self
                .container
                .add_widget::<CounterEdit<T>>((relm.stream().clone(), *unit));
            self.model.counters.push(widget);
        }
    }

    fn update(&mut self, event: CounterMsg<T>) {
        // Pass messages to display
        self.model
            .display
            .as_ref()
            .map(|display| display.stream().emit(event));
    }

    view! {
        #[name="container"]
        gtk::Box {
            spacing: 10,
        }
    }
}

#[derive(Msg)]
pub enum CounterEditMsg {
    Increment,
    Decrement,
    Key(key::Key, i8),
    Scroll(ScrollDirection),
    FlagClear,
    Submit,
}

pub struct CounterEditModel<T>
where
    T: Unit,
{
    upstream: EventStream<CounterMsg<T>>,
    unit: T,
    key_flag: bool,
}

#[widget]
impl<T> Widget for CounterEdit<T>
where
    T: Unit,
{
    fn model((upstream, unit): (EventStream<CounterMsg<T>>, T)) -> CounterEditModel<T> {
        CounterEditModel {
            upstream,
            unit,
            key_flag: false,
        }
    }

    fn init_view(&mut self) {
        self.entry
            .widget()
            .set_placeholder_text(Some(self.model.unit.as_static()));
        self.entry.widget().add_events(EventMask::SCROLL_MASK);

        self.inc.add_events(EventMask::BUTTON_PRESS_MASK);
        self.dec.add_events(EventMask::BUTTON_PRESS_MASK);
    }

    fn update(&mut self, event: CounterEditMsg) {
        match event {
            CounterEditMsg::Submit => {
                if let Some(text) = self.entry.widget().get_text() {
                    let msg = CounterMsg::Change(text.to_string(), self.model.unit);
                    self.model.upstream.emit(msg);
                }
                self.entry.widget().set_text("");
            }
            CounterEditMsg::Increment | CounterEditMsg::Scroll(ScrollDirection::Up) => {
                if self.model.key_flag {
                    self.model.key_flag = false;
                } else {
                    self.model
                        .upstream
                        .emit(CounterMsg::Increment(self.model.unit));
                }
            }
            CounterEditMsg::Decrement | CounterEditMsg::Scroll(ScrollDirection::Down) => {
                if self.model.key_flag {
                    self.model.key_flag = false;
                } else {
                    self.model
                        .upstream
                        .emit(CounterMsg::Decrement(self.model.unit));
                }
            }
            CounterEditMsg::Key(key, direction) => {
                let value = match key {
                    key::Return => 1,
                    key::_1 => 1,
                    key::_2 => 2,
                    key::_3 => 3,
                    key::_4 => 4,
                    key::_5 => 5,
                    key::_6 => 6,
                    key::_7 => 7,
                    key::_8 => 8,
                    key::_9 => 9,
                    key::_0 => 10,
                    _ => 0,
                };
                if value != 0 {
                    self.model
                        .upstream
                        .emit(CounterMsg::Add(direction as i64 * value, self.model.unit));
                    self.model.key_flag = true;
                }
            }
            CounterEditMsg::FlagClear => self.model.key_flag = false,
            _ => {}
        }
    }

    view! {
        #[name="container"]
        gtk::Box {
            orientation: gtk::Orientation::Vertical,
            spacing: 10,
            margin_top: 10,
            margin_bottom: 10,
            margin_start: 10,
            margin_end: 10,

            #[name="entry"]
            TextEntry {
                scroll_event(_, event) => (CounterEditMsg::Scroll(event.get_direction()), Inhibit(false)),
                activate => CounterEditMsg::Submit,
                input_purpose: gtk::InputPurpose::Number,
            },
            gtk::Box {
                hexpand: true,
                halign: gtk::Align::Center,

                #[name="dec"]
                gtk::Button {
                    label: "-",
                    margin_end: 5,
                    key_press_event(_, event) => (CounterEditMsg::Key(event.get_keyval(), -1), Inhibit(false)),
                    leave_notify_event(_, _) => (CounterEditMsg::FlagClear, Inhibit(false)),
                    clicked => CounterEditMsg::Decrement,
                },

                #[name="inc"]
                gtk::Button {
                    label: "+",
                    margin_start: 5,
                    key_press_event(_, event) => (CounterEditMsg::Key(event.get_keyval(), 1), Inhibit(false)),
                    leave_notify_event(_, _) => (CounterEditMsg::FlagClear, Inhibit(false)),
                    clicked => CounterEditMsg::Increment,
                },
            }
        }
    }
}
