use std::collections::vec_deque::VecDeque;
use bffcore;
use bffcore::engine::bfo_reader::BFOReader;

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    let _path = args.pop_front().unwrap();
    let mut file_path = "./assembly/main.bfo".to_string();
    let mut debug = false;

    if args.len() > 0 && !args[0].starts_with("-"){
        file_path = args.pop_front().unwrap();
        if !file_path.ends_with(".bfo") {
            eprintln!("Warning: Given File is not a '.bfo' file");
        }
    } else {
        eprintln!("Warning: Default File not found, using '{}' instead", file_path);
    }

    while args.len() != 0 {
        let current_arg = args.pop_front().unwrap();

        if current_arg.starts_with("-"){
            match &*current_arg {
                "--debug" | "-d" => {
                    debug = true;
                }
                unknown => {
                    unimplemented!("Unknown Argument: '{}'", unknown)
                }
            }
        }
    }

    let contents = match std::fs::read(file_path.clone()){
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Couldnt read {}", file_path)
        }
    };

    let mut bff_program = BFOReader::read_program(contents);
    bff_program.execute();
}