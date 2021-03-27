mod slicer_types;
mod util;

use slicer_types::*;
use util::*;

fn main() {
    // Getting Command Line Args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    // Checking for correct amount of Args
    if !(2..=3).contains(&args.len()) {
        println!("Usage: <input dir> <output dir> [<leftover dir>]");
        std::process::exit(0);
    }

    // formating folder paths
    if !args[0].ends_with("/") {
        args[0] = format!("{}{}", args[0], "/")
    }
    if !args[1].ends_with("/") {
        args[1] = format!("{}{}", args[1], "/")
    }
    if args.len() == 3 {
        if !args[2].ends_with("/") {
            args[2] = format!("{}{}", args[2], "/")
        }
    }

    // Parsing Args
    let in_path = args[0].clone();
    let out_path = args[1].clone();
    let mut leftover = None;

    // Creates Left Over Dir if given
    if args.len() == 3 {
        let lo = args[2].as_str();
        create_texture_pack_dir(lo, &PackDirType::GUI);
        leftover = Some(lo);
    }

    create_texture_pack_dir(&out_path, &PackDirType::NONE);

    slice_paintings(&in_path, &out_path, leftover);
    slice_effects(&in_path, &out_path, leftover);
    slice_particles(&in_path, &out_path, leftover);
    slice_explosion(&in_path, &out_path, leftover);
    slice_sweep(&in_path, &out_path, leftover);
}
