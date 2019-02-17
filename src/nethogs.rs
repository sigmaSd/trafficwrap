fn nethogs() {
    nethogs();
    let mut map = vec!();
    loop {
        map.clear();
        let mut file = fs::File::open("/tmp/t").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut result = String::new();
        for line in contents.lines().rev() {
            if line.contains("Refreshing:") {
                break
            }
            result.push_str(line);
            result.push('\n');
        }
        result.lines().for_each(|l| {
            let l = l.split('\t').map(|c|c.to_owned()).collect::<Vec<String>>();
            map.push(l);
        });
        println!("{:?}", &map);
    }
}

fn hid_nethogs() {
    let t = fs::File::create("/tmp/t").unwrap();
    std::process::Command::new("nethogs")
        .arg("-t")
        .stdout(t)
        .spawn()
        .expect("ls command failed to start");


}

