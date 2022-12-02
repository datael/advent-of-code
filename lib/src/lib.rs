use std::io::*;

pub fn read_all_lines<R: BufRead>(reader: R) -> impl IntoIterator<Item = String> {
    reader.lines().flatten()
}

pub fn read_all_lines_from_stdin() -> impl IntoIterator<Item = String> {
    read_all_lines(stdin().lock())
}
