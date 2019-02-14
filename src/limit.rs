use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/*fn main() {
    let (program, download, upload) = parse();
    limit(program, download, upload);
}*/

/*fn parse() -> (String, String, Option<String>) {
    let cmd_args: Vec<String> = args().skip(1).collect();
    let upload = if cmd_args.len() == 4 {
        Some(cmd_args[3].clone())
    } else {
        None
    };
    (cmd_args[0].clone(), cmd_args[1].clone(), upload)
}*/
pub struct Limiter {
    state: Arc<Mutex<HashMap<String, String>>>,
}
impl Limiter {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn limit(&mut self, p: &str, d: String, u: Option<String>) {
        let p = p.to_owned();
        let p1 = p.clone();
        //let (tx, rx) = mpsc::channel();
        let state = Arc::clone(&self.state);
        thread::spawn(move || {
            stop();
            let mut shell = String::from(
                "
    app:
        download: dskbps
        upload: uskbps
        match:
            - exe: path",
            );

            shell = shell.replace("app", &p1.clone().split('/').last().unwrap());
            shell = shell.replace("ds", &d);
            if u.is_some() {
                shell = shell.replace("us", &u.unwrap());
            } else {
                shell = shell.replace("upload: uskbps", "");
            }

            shell = shell.replace("path", &p1);
            //tx.send(shell);
            state.lock().unwrap().insert(p.clone(), shell);

            Self::execute(state);
        });
        //let shell = rx.try_recv().unwrap();
    }

    fn execute(state: Arc<Mutex<HashMap<String, String>>>) {
        let mut shell: String = state
            .lock()
            .unwrap()
            .values()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        shell.insert_str(0, "processes:\n");
        let file = File::create("/tmp/traffic_conf").unwrap();
        write!(&file, "{}", shell).unwrap();

        Command::new("sudo")
            .current_dir("/tmp")
            .args(&["tt", "wlp3s0", "traffic_conf"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

pub fn list_process() -> Vec<String> {
    let processs = Command::new("ps").arg("aux").output().unwrap();
    let mut seen_process = vec![];

    let mut process_list: Vec<(String, String)> = String::from_utf8(processs.stdout)
        .unwrap()
        .lines()
        .filter_map(|s| {
            let s_cpu = s.split_whitespace().nth(3).unwrap();
            let s_path = s.split_whitespace().nth(10).unwrap();

            if !seen_process.contains(&s_path) && s_path.starts_with('/') && s_cpu != "0.0" {
                seen_process.push(s_path);
                Some((s_path.to_string(), s_cpu.to_string()))
            } else {
                None
            }
        })
        .collect();

    // remove duplicates
    //--------------
    //let set: HashSet<_> = process_list.drain(..).collect(); // dedup
    //process_list.extend(set.into_iter());
    //--------------

    process_list.sort_by(|(_, k), (_, k2)| {
        k.parse::<f32>()
            .unwrap()
            .partial_cmp(&k2.parse::<f32>().unwrap())
            .unwrap()
    });
    //dbg!(&process_list);

    process_list.into_iter().map(|(p, _)| p).rev().collect()
}
pub fn stop() {
    Command::new("sudo")
        .args(&["kill", "-2", "tt"])
        .output()
        .unwrap();
}

/*fn which(p: String) -> String {
    String::from_utf8(Command::new("which").arg(&p).output().unwrap().stdout).unwrap()
}*/

// tw firefox 300
