use std::collections::vec_deque::VecDeque;
use bff;

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    let _path = args.pop_front().unwrap();
    let mut file_path = "main.bfo".to_string();
    let mut debug = false;

    if args.len() > 0 && !args[0].starts_with("-"){
        file_path = args.pop_front().unwrap();
        if !file_path.ends_with(".bfo") {
            println!("Warning: Given File is not a '.bfo' file");
        }
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
}