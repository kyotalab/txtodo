use crate::model::Todo;

use std::fs::OpenOptions;
use std::io::{BufWriter, Error, Write};

pub fn write_to_toto_txt(todo: &Todo) -> Result<(), Error> {
    let output = format!("{todo}\n");

    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open("todo.txt")
        .unwrap();
    let mut bw = BufWriter::new(f);

    bw.write(output.as_bytes()).unwrap();
    Ok(())
}
