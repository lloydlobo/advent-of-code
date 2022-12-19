//! # Calorie Counting. Day 1
//!
//! # Part 1
//!
//! By the time you calculate the answer to the Elves' question, they've already
//! realized that the Elf carrying the most Calories of food might eventually
//! run out of snacks.
//! To avoid this unacceptable situation, the Elves would instead like to know
//! the total Calories carried by the top three Elves carrying the most
//! Calories. That way, even if one of those Elves runs out of snacks, they
//! still have two backups.
//!
//! * Find the Elf carrying the most Calories.
//! * How many total Calories is that Elf carrying?
//!
//! # Part 2
//!
//! In the example above, the top three Elves are the fourth Elf (with 24000
//! Calories), then the third Elf (with 11000 Calories), then the fifth Elf
//! (with 10000 Calories). The sum of the Calories carried by these three elves
//! is 45000.
//!
//! * Find the top three Elves carrying the most Calories.
//! * How many Calories are those Elves carrying in total?

use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, Read, Write},
    result::Result,
    string::String,
};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: u32,
    y: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Elf {
    index: u32,
    load: u32,
}

const FILE_OUT: &str = "out.json";

/// This is a multi-line Google style docstring.
///
/// It is used to document the function.
///
/// # Examples
///
/// ```
/// let (haystack, needle) = read_from_file();
/// part_1(haystack, needle);
/// part_2(haystack, needle);
/// ```
fn main() {
    SerdeJson::set_point();
    let data_in = FsIo::read();
    match data_in {
        Ok(result) => {
            println!("main: Success: {:?}", result);
            let y: Result<(), Error> = FsIo::write(&result);
            println!("main: Success: {:?}", y);
        }
        Err(e) => {
            println!("main: Error: {:?}", e);
        }
    };
    let (haystack, needle) = read_from_file();
    part_1(haystack, needle);
    part_2(haystack, needle);
} // Thanks to Chris Biscardi.

struct FsIo;

impl FsIo {
    /// https://doc.rust-lang.org/std/fs/struct.File.html
    pub fn read() -> Result<String, Error> {
        let file = include_str!(r#"in.txt"#);
        let mut buf_reader = BufReader::new(file.as_bytes());

        let mut contents = String::new();
        match buf_reader.read_to_string(&mut contents) {
            Ok(it) => {
                println!("bytes read: {:?}", it); // 13
                it
            }
            Err(err) => {
                println!("Error: {:?}", err);
                return Err(err);
            }
        };
        // assert_eq!(contents, "Hello, world!");
        Ok(contents)
    }

    /// writing `&String` instead of `&str` involves a new object where a slice will do.
    pub fn write(contents: &str) -> Result<(), Error> {
        let binding = String::from(contents);

        let copy = binding
            .split('\n')
            .par_bridge() // Creates a bridge from this type to a `ParallelIterator`..
            .into_par_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        println!("copy :: {:?}", copy);

        // Write `copy` to stdout.
        let mut buf_writer = BufWriter::new(File::create(FILE_OUT)?);
        match buf_writer.write_all(copy.join("\n").as_bytes()) {
            Ok(it) => {
                println!("bytes written: {:?}", it); // 13
                                                     // Ok(())
            }
            Err(err) => {
                println!("Error: {:?}", err);
                // Err(err)
            }
        };

        let got = contents.split('\n').collect::<String>();
        let json = serde_json::to_string(&got).unwrap();
        let file = File::create(FILE_OUT)?;
        let mut bufwr: BufWriter<File> = BufWriter::new(file);
        match bufwr.write_all(json.as_bytes()) {
            Ok(it) => {
                println!("bytes written: {:?}", it); // 13
                println!("Ok() write_all: {:?}", json);
                it
            }
            Err(err) => return Err(err),
        };
        Ok(())
    }
}

struct SerdeJson;

impl SerdeJson {
    pub fn set_point() {
        let point: Point = Point { x: 1, y: 2 };
        // Convert point to JSON.
        let point: String = serde_json::to_string(&point).unwrap();
        // Print serialized.
        println!("serialized = {}", point);
        // Convert JSON back to point.
        let point: Point = serde_json::from_str(&point).unwrap();
        // Print deserialized.
        println!("deserialized = {:?}", point);
    }
}

/// Find the top three Elves carrying the most Calories. How many Calories are
/// those Elves carrying in total? */
///
/// # Examples
///
/// ```
/// use day_1_calorie_counting::part_2;
/// let haystack = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
/// let  needle = "\n\n";
/// assert_eq!(part_2(haystack, needle), 24_000_u32);
/// ```
pub fn part_2(haystack: &str, needle: &str) -> u32 {
    let mut food = haystack
        .split(needle)
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|calorie| calorie.parse::<u32>().unwrap()) // .sorted_by(|a, b| b.cmp(a))
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    // food.sort() sorts greatest num at the back but we want top 3.
    food.sort_by(|a, b| b.cmp(a)); // Expensive computation on mutable.
    food.iter().take(3).sum::<u32>() // Iter on the Vec first 3 elements and sum all 3.
}

/// Find the Elf carrying the most Calories. **How many total Calories is that
/// Elf carrying?**
///
/// # Examples
///
/// ```
/// use day_1_calorie_counting::part_1;
/// let haystack = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
/// let  needle = "\n\n";
/// assert_eq!(part_1(haystack, needle), 45_000_u32);
/// ```
pub fn part_1(haystack: &str, needle: &str) -> u32 {
    let food = haystack
        .split(needle) // chunk up lines between each empty line;
        .map(|elf_load| {
            elf_load
                .lines() //  OR use .split('\n')) split each chunk for single line.
                .map(|item| item.parse::<u32>().unwrap()) // Assure a valid u32 or crash Whenever a string (usually an empty line) can;t be parsed return error and skip.// Parse string into u32 to Result.
                .sorted_by(|a, b| b.cmp(a)) // Sort descending.cmp , Ordering::Less.
                .sum::<u32>() // Sum all the values into u32 with turbo fish syntax.
        })
        .max() // Max returns an Result.
        .unwrap(); // If there is 1 or no so, unwrap to see if there is a value.
    food
}

/// Reads the file and returns a tuple of the file contents and the needle
///
/// # Examples
///
/// ```
/// let (haystack, needle) = read_from_file();
/// assert_eq!(result, "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
/// ```
fn read_from_file() -> (&'static str, &'static str) {
    let haystack = include_str!("day_1_calorie_counting.txt"); // let haystack: &str = get_testcase();
    let needle: &str = "\n\n";
    (haystack, needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// # Examples
    ///
    /// ```
    /// use aoc_2019_rust::day_2::part_1;
    ///
    /// let (haystack, needle) = read_from_file();
    /// assert_eq!(part_1(haystack, needle), 71_780_u32);
    /// ```
    fn test_part_1() {
        let (haystack, needle) = read_from_file();
        assert_eq!(part_1(haystack, needle), 71_780_u32);
    }

    #[test]
    /// # Examples
    ///
    /// ```
    /// use aoc_2019_rust::day_2::part_2;
    ///
    /// let (haystack, needle) = read_from_file();
    /// assert_eq!(part_2(haystack, needle), 212_489_u32);
    /// ```
    fn test_part_2() {
        let (haystack, needle) = read_from_file();
        assert_eq!(part_2(haystack, needle), 212_489_u32);
    }

    #[test] // let cow = Cow::from(&split.clone().collect::<String>());
    fn test_read_from_file() {
        let (haystack, needle) = read_from_file();
        assert_eq!(needle, "\n\n");

        let split = haystack.split(needle);
        assert_eq!(split.clone().count(), 254_usize);

        let no_empty_lines = split
            .map(|lines| lines.lines().count())
            .filter(|&count| count == 0)
            .count();
        assert_eq!(no_empty_lines, 0_usize);
    }

    #[test]
    /// This is a multi-line Google style docstring.
    ///
    /// It is used to document the function.
    ///
    /// # Examples
    ///
    /// ```
    /// let (haystack, needle) = (get_testcase(), "\n\n");
    /// assert_eq!(part_1(haystack, needle), 24_000_u32);
    /// ```
    fn test_part_1_example() {
        let (haystack, needle) = (get_testcase(), "\n\n");
        assert_eq!(part_1(haystack, needle), 24_000_u32);
    }

    #[test]
    /// This is a multi-line Google style docstring.
    ///
    /// It is used to document the function.
    ///
    /// # Examples
    ///
    /// ```
    /// let (haystack, needle) = (get_testcase(), "\n\n");
    /// assert_eq!(part_2(haystack, needle), 45_000_u32);
    /// ```
    fn test_part_2_example() {
        let (haystack, needle) = (get_testcase(), "\n\n");
        assert_eq!(part_2(haystack, needle), 45_000_u32);
    }

    /// This is a multi-line Google style docstring.
    ///
    /// It is used to document the function.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = get_testcase();
    /// assert_eq!(result, "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
    /// ```
    fn get_testcase() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }
}
