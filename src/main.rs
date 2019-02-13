use gtk::*;
//use std::process::Command;
//use std::cell::RefCell;

mod limit;
use limit::{limit, list_process, stop};

//const COL: i32 = 0;
//const MARGIN: i32 = 200;
fn main() {
    gtk::init().unwrap();

    let process = list_process();

    let vbox = create_box("v");
    let scrolled_win = ScrolledWindow::new(None, None);
    scrolled_win.add(&vbox);

    for p in process.into_iter() {
        let hbox = create_box("h");
        let btn = SpinButton::new_with_range(0.0, 1000.0, 100.0);
        let p2: String = p.clone().to_owned();
        btn.connect_value_changed(move |btn| {
            limit(&p2, btn.get_value().to_string(), None);
        });
        btn.set_orientation(Orientation::Vertical);
        let label = Label::new(p.as_str());
        hbox.pack_start(&label, true, true, 10);
        hbox.pack_start(&btn, true, true, 10);

        vbox.add(&hbox);
    }

    let win = Window::new(WindowType::Toplevel);
    win.add(&scrolled_win);
    win.resize(500, 400);
    win.set_title("TrafficWrap");
    win.connect_delete_event(|_, _| {
        stop();
        gtk::main_quit();
        Inhibit(false)
    });
    win.show_all();

    gtk::main();
}

fn create_box(o: &str) -> Box {
    match o {
        "v" => Box::new(Orientation::Vertical, 10),
        "h" => Box::new(Orientation::Horizontal, 10),
        _ => unreachable!("box or"),
    }
}
