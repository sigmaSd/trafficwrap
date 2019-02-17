use gtk::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::*;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

mod limit;
use limit::{stop, Limiter};

fn main() {
    gtk::init().unwrap();

    hid_nethogs();
    let mut map = vec![];
    let vbox = create_box("v");
    vbox.set_margin_start(20);
    let win = Window::new(WindowType::Toplevel);
    let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    thread::spawn(move || {
        let s = sender.clone();
        loop {
            thread::sleep(Duration::from_secs(3));
            map.clear();
            let mut file = fs::File::open("/tmp/t").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut result = String::new();
            for line in contents.lines().rev() {
                if line.contains("Refreshing:") {
                    break;
                }
                if line.starts_with('/') {
                    result.push_str(line);
                    result.push('\n');
                }
            }
            result.lines().for_each(|l| {
                let l = l.split('\t').map(|c| c.to_owned()).collect::<Vec<String>>();
                map.push(l);
            });

            let _ = s.send(Message::UpdateBox(map.clone()));
        }
    });

    let limiter = Limiter::new();
    let ref_limiter = Rc::new(RefCell::new(limiter));

    let speed_map = Rc::new(RefCell::new(HashMap::new()));

    let mut vbox_c = vbox.clone();
    receiver.attach(None, move |msg| {
        match msg {
            Message::UpdateBox(p_list) => {
                clear_box(&mut vbox_c);
                for p in p_list {
                    let hbox = create_box("h");

                    let exe = {
                        let p = p[0].to_string();
                        let a = p.rfind('/').unwrap();
                        let (p, _) = p.split_at(a);
                        let a = p.rfind('/').unwrap();
                        let (p, _) = p.split_at(a);
                        p.to_string()
                    };
                    let label1 = Label::new(exe.as_str());
                    let label2 = Label::new(p[1].clone().as_str());
                    let label3 = Label::new(p[2].clone().as_str());
                    hbox.add(&label1);
                    hbox.add(&label2);
                    hbox.add(&label3);

                    let btn = SpinButton::new_with_range(0.0, 1000.0, 100.0);
                    if let Some(s) = speed_map.borrow().get(&exe) {
                        btn.set_value(*s);
                    }

                    let lc = ref_limiter.clone();
                    let sm = speed_map.clone();
                    btn.connect_value_changed(move |btn| {
                        sm.borrow_mut().insert(exe.to_string(), btn.get_value());
                        lc.borrow_mut()
                            .limit(&exe, btn.get_value().to_string(), None);
                    });
                    btn.set_orientation(Orientation::Vertical);

                    hbox.add(&btn);

                    vbox_c.add(&hbox);
                    vbox_c.show_all();
                }
            }
        }

        glib::Continue(true)
    });
    //let scrolled_win = gtk::ScrolledWindow::new(None, None);
    //scrolled_win.add(&vbox);
    win.add(&vbox);

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

fn hid_nethogs() {
    let t = fs::File::create("/tmp/t").unwrap();
    std::process::Command::new("nethogs")
        .arg("-t")
        .stdout(t)
        .spawn()
        .expect("ls command failed to start");
}

fn create_box(o: &str) -> Box {
    match o {
        "v" => Box::new(Orientation::Vertical, 10),
        "h" => Box::new(Orientation::Horizontal, 10),
        _ => unreachable!("box or"),
    }
}

#[derive(Clone)]
enum Message {
    UpdateBox(Vec<Vec<String>>),
}

fn clear_box(b: &mut Box) {
    let cs = b.get_children();
    for c in cs {
        b.remove(&c);
    }
}
