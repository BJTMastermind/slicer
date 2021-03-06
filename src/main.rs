mod util;

use util::*;

fn main() {
    // Getting Command Line Args
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    // Checking for correct amount of Args
    if !(2..=3).contains(&args.len()) {
        println!("Usage: <input dir or zip> <output dir> [<leftover dir>]");
        std::process::exit(0);
    }

    // Check to make sure output/leftover path is not a zip.
    if args[1].ends_with(".zip") {
        println!("Output path can't be a zip file.\n");
        println!("Usage: <input dir or zip> <output dir> [<leftover dir>]");
        std::process::exit(0);
    }
    if args[2].ends_with(".zip") {
        println!("Leftover path can't be a zip file.\n");
        println!("Usage: <input dir or zip> <output dir> [<leftover dir>]");
        std::process::exit(0);
    }

    // formating folder paths
    for i in 0..3 {
        if let Some(arg) = args.get_mut(i) {
            #[cfg(unix)]
            if !arg.ends_with(".zip") && !arg.ends_with("/") {
                *arg = format!("{}{}", arg, "/")
            }
            #[cfg(windows)]
            if !arg.ends_with(".zip") && !arg.ends_with("\\") {
                *arg = format!("{}{}", arg, "\\")
            }
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

    if in_path.ends_with(".zip") {
        read_from_zip(&in_path, &out_path.as_str(), leftover);       
    } else {
        ImageData::default().slice_all(&in_path, &out_path, leftover);
    }    
}
