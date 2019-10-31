use gdk::enums::key;
use gtk::{EditableExt, EntryExt, Inhibit, WidgetExt};
use relm::{connect, connect_stream, Widget};
use relm_derive::{widget, Msg};
use std::fmt::Display;

pub struct TextEditModel {
    // view: Relm<TextDisplayView>,
}

#[derive(Msg)]
pub enum TextEditMsg {
    SelectAll,
    Submit,
    Key(key::Key),
}

#[widget]
impl Widget for TextEntry {
    fn model() -> TextEditModel {
        TextEditModel {}
    }

    fn update(&mut self, event: TextEditMsg) {
        match event {
            TextEditMsg::SelectAll => self.entry.select_region(0, -1),
            TextEditMsg::Submit => self.entry.select_region(0, -1),
            TextEditMsg::Key(key) => {
                match key {
                    key::Escape => self.entry.get_text().map(|t| self.entry.set_text(&t)),
                    _ => None,
                };
            }
        }
    }

    view! {
        gtk::EventBox {
            focus_in_event(_, _) => (TextEditMsg::SelectAll, Inhibit(false)),
            // focus_out_event(_, _) => (TextEditMsg::FocusIn, Inhibit(false)),
            key_press_event(_, event) => (TextEditMsg::Key(event.get_keyval()), Inhibit(false)),

            #[name="entry"]
            gtk::Entry {
                focus_out_event(_, _) => (TextEditMsg::Submit, Inhibit(false)),
            }
        }
    }
}

pub trait Markup: Display {
    fn markup_xx_large(&self) -> String {
        format!("<span font_size=\"xx-large\">{}</span >", self)
    }

    fn markup_title(&self) -> String {
        self.markup_xx_large().markup_bold()
    }

    fn markup_bold(&self) -> String {
        format!("<b>{}</b>", self)
    }

    fn markup_fontsize(&self, size: usize) -> String {
        format!("<span font=\"{}\">{}</span>", size, self)
    }
}

impl Markup for String {}
impl Markup for str {}
