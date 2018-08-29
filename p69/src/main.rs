use std::collections::{HashMap};
use std::io::{BufReader, BufRead, Write};
use std::fs::{File};

struct LineFreq<'a> {
    map: HashMap<&'a str, usize>,
}

impl<'a> LineFreq<'a> {
    pub fn new<'b>() -> LineFreq<'b> {
        const NOT_BROKEN: &str = "NKFz87zGEacY1R4DCQ81iWMjABK+slCD93F6pf3CLM+Rh1wjDk3Be8L0T5KTsEveQaqWNO6j";

        let mut map = HashMap::new();
        map.insert(NOT_BROKEN, 1);
        LineFreq { map }
    }

    pub fn add_line(&mut self, line: &'a str) {
        let entry = self.map.entry(line).or_insert(0);
        *entry += 1;
    }

    pub fn is_authentic(&self, line: &'a str) -> bool {
        *self.map.get(line).unwrap_or(&0) > 1
    }
}

fn main() {
    let raw_file = File::open("cyberpunk2077-raw.b64")
        .expect("couldn't open raw file");
    let all: Vec<String> = BufReader::new(raw_file).lines().map(Result::unwrap).collect();
    let mut files = Vec::<Vec<&str>>::new();
    let mut file = Vec::<&str>::new();
    let mut lf = LineFreq::new();

    for line in all.iter() {
        if line.starts_with("new file!") {
            if !file.is_empty() {
                files.push(file.split_off(0));
            }
        } else {
            if line.is_empty() || line.len() != 72 || line.contains(' ') {
                continue;
            }
            lf.add_line(line);
            file.push(line);
        }
    }

    let mut goodfiles = Vec::<Vec<&str>>::new();
    goodfiles.push(files.remove(0));
    for file in files.into_iter() {
        let file_repeats = {
            let last_file = goodfiles.last().unwrap();
            &last_file[0..3] == &file[0..3]
        };
        if !file_repeats {
            goodfiles.push(file);
        }
    }
    files = goodfiles;

    let mut res = Vec::<&str>::new();
    let mut res2 = Vec::<String>::new();
    for line in &files[0][0..21] {
        res.push(line);
    }

    for (i, file) in files.iter().enumerate() {
        let miol = {
            let last = res.last().unwrap();
            file.iter().enumerate().find(|(_, x)| x == &last)
        };
        if let Some((iol, _)) = miol {
            let rest = file[(iol+1)..].iter()
                .take_while(|line| lf.is_authentic(line));
            for line in rest {
                if res.last() != Some(line) {
                    res.push(line);
                }
            }
        } else {
            let last = res.last().unwrap();
            res2.push(format!("Skipped file {0}, not found {1}", i, last));
        }
    }

    // some hardcoded lines at the end of the file
    res.push("kv8/70Ku4e70cWYAAAAASUVORK5CYII=");

    let mut out = File::create("cyberpunk2077-decoded.png.b64")
        .expect("couldn't open output file");
    for line in res {
        writeln!(out, "{}", line);
    }
    // File.WriteAllLines("whateverout2-difficult.txt", res2);
}
