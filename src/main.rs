use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    // include_str!("path") gibt &str des textes zurück
    if let Ok(lines) = read_lines("src/input.txt") {
        let mut counter = 0;
        let mut data: Vec<i32> = Vec::new();

        for line_or_null in lines {
            if let Ok(line) = line_or_null {
                data.push(line.parse().unwrap());
            }
        }

        let mut prev_sum = data[0..3].iter().sum();
        for i in 1..data.len() - 2 {
            let curr_sum: i32 = data[i..i + 3].iter().sum();
            if curr_sum > prev_sum {
                counter += 1;
            }
            prev_sum = curr_sum;
        }
        println!("{}", counter);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
