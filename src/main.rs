use gtk::*;
use std::cell::RefCell;
use std::rc::Rc;

mod limit;
use limit::{list_process, stop, Limiter};

fn main() {
    gtk::init().unwrap();

    let limiter = Limiter::new();
    let ref_limiter = Rc::new(RefCell::new(limiter));
    let process = list_process();

    let vbox = create_box("v");
    let scrolled_win = ScrolledWindow::new(None, None);
    scrolled_win.add(&vbox);

    for p in process.into_iter() {
        let hbox = create_box("h");
        let btn = SpinButton::new_with_range(0.0, 1000.0, 100.0);
        let p2: String = p.clone();
        let lc = ref_limiter.clone();
        btn.connect_value_changed(move |btn| {
            lc.borrow_mut()
                .limit(&p2, btn.get_value().to_string(), None);
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
