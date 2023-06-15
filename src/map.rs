use std::{fs::File, io::{BufReader, BufRead}};

pub struct Map {
    layout: String,
    w: usize,
    h: usize,
    tsize: usize
}

impl Map {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut layout: String = String::new();
        let mut w: usize = 0;
        let mut h: usize = 0;
        for line in reader.lines() {
            let s: String = line.unwrap();
            layout.push_str(s.as_str());

            h += 1;
            w = s.len();
        }

        Self {
            layout,
            w,
            h,
            tsize: 50
        }
    }

    pub fn at(&self, x: usize, y: usize) -> char {
        self.layout.chars().nth(y * self.w + x).unwrap()
    }
}

