# slicer

## About This Project

Slicer is a Rust port of Mojang's [slicer](https://github.com/Mojang/slicer) program written in Java.

## How To Compile [Optional]

1. Open Terminal/Command Prompt and cd to the project directory. (outside the src folder)
2. Type ``cargo build --release``
3. Go into the generated target folder then into the release folder.
4. The compiled executable file should be in here called `slicer`. Feel free to copy the file anywhere else to use.

## How To Use (Command Line Only)

If using compiled executable file, cd to the directory with the slicer executable<br>
and replace all `cargo run --release --` with `./slicer` when running the commands<br>

If any of the folder/file paths contain space's make sure to put the paths in double quotes<br>
like this `"./path/to my/cool pack"`

``cargo run --release -- <input dir or zip> <output dir> [<leftover dir>]`` (``leftover dir`` is optional location that will be filled with copies of source images with added highlights for no longer needed areas).

## Example Commands
``cargo run --release -- ./My-Cool-Pack ./output``<br>
``cargo run --release -- ./My-Cool-Pack ./output ./leftovers``<br>
``cargo run --release -- ./My-Cool-Pack ./My-Cool-Pack``<br>
``cargo run --release -- ./My-Cool-Pack ./My-Cool-Pack ./leftovers``<br>

Output directory can't be the same as the input if the input is a zip.<br>
``cargo run --release -- ./My-Cool-Pack.zip ./output ./leftovers``
