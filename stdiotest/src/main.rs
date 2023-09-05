use std::io::BufRead;

fn get_lines_from_stdin() -> Option<Vec<String>> {
    atty::is(atty::Stream::Stdin).then(|| {
        std::io::stdin()
            .lock()
            .lines()
            .collect::<Result<_, _>>()
            .expect("Error reading from stdin")
    })
}

fn main() {
   let linesoptions = get_lines_from_stdin();
    match linesoptions {
        Some(linesVec)=> println!("{}",linesVec[0]),
        None=>println!("{}", "error")
    }
}
