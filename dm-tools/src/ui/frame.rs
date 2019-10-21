use gtk::Orientation::Vertical;
use gtk::{GtkMenuItemExt, MenuButtonExt, OrientableExt, WidgetExt};
use relm::{init, Component, Widget};
use relm_derive::{widget, Msg};

pub struct Model {
    option_menu: Component<Options>,
}

#[derive(Msg)]
pub enum Msg {
    Add,
    Remove,
}

#[widget]
impl Widget for Frame {
    fn model() -> Model {
        let option_menu = init::<Options>(()).expect("Options");

        Model { option_menu }
    }

    fn update(&mut self, event: Msg) {
        match event {
            _ => {}
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,
            #[name="header"]
            gtk::HeaderBar {
                gtk::MenuButton {
                    halign: gtk::Align::End,
                    popup: Some(self.model.option_menu.widget()),
                }
            }
        }
    }
}

pub struct OptionModel;

#[derive(Msg)]
pub enum OptionMsg {}

#[widget]
impl Widget for Options {
    fn model() -> OptionModel {
        OptionModel {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            _ => {}
        }
    }

    view! {
        #[name="menu"]
        gtk::Menu {
            gtk::MenuItem {
                label: "Foo"
            },
            gtk::MenuItem {
                label: "Bar"
            }
        }
    }
}
