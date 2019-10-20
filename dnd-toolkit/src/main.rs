mod counter;

use chrono::Local;
use gtk::Orientation::Vertical;
use gtk::{
    Button, ButtonExt, ContainerExt, Inhibit, Label, LabelExt, WidgetExt, Window, WindowType,
};
use relm::EventStream;


// There will be several widgets involved in this example, but this struct
// will act as a container just for the widgets that we will be updating.
struct Widgets {clock_label: Label,
                counter_label: Label,
}

// This enum holds the various messages that will be passed between our
// widgets. Note that we aren't deriving `Msg` because this example uses
// the `core` module, which is the basic event-handling library that
// `relm` depends on.
#[derive(Clone, Debug)]
enum Msg {
    Quit
}

// This struct represents the model, and it maintains the state needed to
// populate the widget. The model is updated in the `update` method.
struct Model {
}


fn main() {
    gtk::init().expect("gtk::init failed");

    let window = Window::new(WindowType::Toplevel);
    let counter_view = counter::init();
    let main_stream = EventStream::new();

    {
        let stream = main_stream.clone();
        window.connect_delete_event(move |_, _| {
            stream.emit(Msg::Quit);
            Inhibit(false)
        });
    }

    window.add(&counter_view);
    window.show_all();

    gtk::main();
}
