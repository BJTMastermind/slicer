mod slicer_types;
mod util;

use slicer_types::*;
use util::*;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if args.len() != 2 && args.len() != 3 {
        println!("Usage: <input dir> <output dir> [<leftover dir>]");
        std::process::exit(0);
    }
    if !args[0].ends_with("/") {args[0] = format!("{}{}", args[0], "/")}
    if !args[1].ends_with("/") {args[1] = format!("{}{}", args[1], "/")}
    if args.len() == 3 { if !args[2].ends_with("/") {args[2] = format!("{}{}", args[2], "/")} }

    let in_path = args[0].clone();
    let out_path = args[1].clone();
    let mut leftover = "".to_string();
    if args.len() == 3 {
        leftover = args[2].clone();
        create_dirs(&leftover, true);
    }

    create_dirs(&out_path, false);
    
    slice_paintings(&in_path, &out_path, &leftover);
    slice_effects(&in_path, &out_path, &leftover);
    slice_particles(&in_path, &out_path, &leftover);
    slice_explosion(&in_path, &out_path, &leftover);
    slice_sweep(&in_path, &out_path, &leftover);
}