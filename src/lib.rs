use std::fs;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file(filename: &str) -> impl Iterator<Item = String> {
    let file = fs::File::open(filename).expect("Couldn't open file");
    let buf_reader = BufReader::new(file);

    buf_reader.lines().map(|x| x.unwrap())
}
