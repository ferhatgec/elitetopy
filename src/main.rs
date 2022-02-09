// MIT License
//
// Copyright (c) 2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::io::{Write};

mod parse;

// function taken from gretea https://github.com/ferhatgec/gretea/blob/master/src/main.rs
fn create_and_write(path: &std::path::Path, generated: String) {
    let mut file = match std::fs::File::create(path) {
        Err(why) => panic!("elitetopy: couldn't write to {}: {}", path.display(), why),
        Ok(file) => file
    };

    match file.write_all(generated.as_bytes()) {
        Err(why) => panic!("elitetopy: couldn't write to {}: {}", path.display(), why),
        _ => {}
    }
}


fn main() {
    let cli_arguments: Vec<_> = std::env::args().collect();

    if cli_arguments.len() < 2 {
        println!("elitetopy - create python scripts from elite\n\
                 ----------\n\
                 {arg} file", arg = cli_arguments.get(0).unwrap());
        std::process::exit(1);
    }

    let mut elite_read = elite::read::EliteFileData {
        raw_data: elite::read::elite_file_contents::create_empty_string(),
        unparsed: vec![]
    };

    elite_read.read_raw_file(cli_arguments.get(1).unwrap());
    let x = elite::lexer::elite_lexer::init_lexer(&elite_read, true);
    let y = parse::elite_python::parse(x);
    let z = format!("{}_out.py", cli_arguments.get(1).unwrap()).as_str().to_owned();
    let path = std::path::Path::new(&z);
    create_and_write(path, y);
}
