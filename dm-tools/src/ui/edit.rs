use gdk::enums::key;
use gtk::{Inhibit, OrientableExt, RevealerExt, WidgetExt};
use relm::{connect, connect_stream, init, Component, ContainerWidget, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum EditMsg {
    SelectAll,
    Submit,
    SetValue(String),
    Key(key::Key),
    ToggleReveal,
}

pub struct EditModel<T, E>
where
    T: Widget<ModelParam = Option<Component<E>>> + 'static,
    E: Widget<ModelParam = Option<Component<T>>> + 'static,
{
    display: Component<T>,
    editor: Component<E>,
}

#[widget]
impl<T, E> Widget for EditView<T, E>
where
    T: Widget<ModelParam = Option<Component<E>>> + 'static,
    E: Widget<ModelParam = Option<Component<T>>> + 'static,
{
    fn model() -> EditModel<T, E> {
        EditModel {
            display: init::<T>(None).expect("Display"),
            editor: init::<E>(None).expect("Editor"),
        }
    }

    fn init_view(&mut self) {
        self.model.display = self.view.add_widget::<T>(Some(self.model.editor.clone()));
        self.model.editor = self.edit.add_widget::<E>(Some(self.model.display.clone()));
    }

    fn update(&mut self, event: EditMsg) {
        match event {
            EditMsg::ToggleReveal => self.edit.set_reveal_child(!self.edit.get_reveal_child()),
            _ => {}
        }
    }

    view! {
        gtk::EventBox {
            key_press_event(_, event) => (EditMsg::Key(event.get_keyval()), Inhibit(false)),


            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                #[name="view"]
                gtk::EventBox {
                    button_press_event(_, _) => (EditMsg::ToggleReveal, Inhibit(false)),
                },

                #[name="edit"]
                gtk::Revealer {}
            }
        }
    }
}
