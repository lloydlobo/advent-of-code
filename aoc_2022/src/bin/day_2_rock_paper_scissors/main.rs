use std::{
    fs,
    io::{self, BufWriter},
    ops::Deref,
    str::from_utf8,
};

use rayon::prelude::{
    IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator,
};
use serde::Serialize;

fn main() {
    main_run();
}

fn get_testcase() -> &'static str {
    "A Y\nB X\nC Z"
}

#[derive(Debug, Serialize)]
struct Input {
    pub lines: Vec<String>,
}

/// let lines = data.iter().flat_map(|line| line.lines().map(|x| x.split::<char>(' ').collect::<Vec<&str>>().concat::<_>())).collect::<Vec<_>>();
fn main_run() -> u32 {
    let data: Vec<&str> = get_testcase().split('\n').collect();
    // let data: Vec<&str> = include_str!("input.txt").split('\n').collect();

    let lines = data
        .par_iter()
        .map(|line| {
            line.lines()
                .map(|line| line.split::<char>(' ').collect::<Vec<_>>().concat())
        })
        .flatten_iter()
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&(Input { lines })).unwrap();
    FsIo::write(json.as_bytes());

    3u32
}

struct FsIo;

impl FsIo {
    pub fn write(buf: &[u8]) {
        dbg!(from_utf8(buf).unwrap()); // Convert byte slice back into a string slice from `as_bytes()`.

        let inner = fs::File::create("day_2.json").unwrap();
        let mut buf_writer = BufWriter::new(inner);

        match io::Write::write_all(&mut buf_writer, buf) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_main_run() {
        assert_eq!(2u32, main_run());
    }
}
