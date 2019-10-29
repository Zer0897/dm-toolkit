use gdk::enums::key;
use gtk::{Inhibit, OrientableExt, RevealerExt, WidgetExt};
use relm::{connect, connect_stream, Component, ContainerWidget, Widget};
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
    T: Widget<ModelParam = ()> + 'static,
    E: Widget<ModelParam = Component<T>> + 'static,
{
    display: Option<Component<T>>,
    editor: Option<Component<E>>,
}

#[widget]
impl<T, E> Widget for EditView<T, E>
where
    T: Widget<ModelParam = ()> + 'static,
    E: Widget<ModelParam = Component<T>> + 'static,
{
    fn model() -> EditModel<T, E> {
        EditModel {
            display: None,
            editor: None,
        }
    }

    fn init_view(&mut self) {
        let display = self.view.add_widget::<T>(());
        let editor = self.edit.add_widget::<E>(display.clone());
        self.model.editor = Some(editor);
        self.model.display = Some(display);
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

// impl<T, E> Container for EditView<T, E>
// where
//     T: Widget<Msg = DisplayMsg, ModelParam = ()> + 'static,
//     E: Widget<ModelParam = Relm<Self>> + 'static,
// {
//     type Container = gtk::Revealer;
//     type Containers = ();

//     fn container(&self) -> &Self::Container {
//         &self.edit
//     }

//     fn other_containers(&self) -> Self::Containers {}
// }
