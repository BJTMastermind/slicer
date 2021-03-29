mod util;

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
    for i in 0..3 {
        if let Some(arg) = args.get_mut(i) {
            #[cfg(unix)]
            if !arg.ends_with("/") {
                *arg = format!("{}{}", arg, "/")
            }
            #[cfg(windows)]
            if !arg.ends_with("\\") {
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

    ImageData::default().slice_all(&in_path, &out_path, leftover);
}
