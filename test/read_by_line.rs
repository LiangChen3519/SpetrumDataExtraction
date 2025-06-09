use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub(crate) fn read_by_line(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?; // use ? instead of unwrap()
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?; // handle error properly
        println!("{}", line);
    }
    Ok(())
}