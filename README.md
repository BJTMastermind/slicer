# slicer

## About This Project

This is a Rust port of Mojang's [slicer](https://github.com/Mojang/slicer) program that is written in Java.

## How To Compile [Optional]

1. Open Termainal/Command Prompt and cd to the project directory. (outside the src folder)
2. Type ``cargo build --release``
3. Go into the generated target folder then into the release folder.
4. The compiled executeable file should be in here called `slicer`. Feel free to copy the file anywhere else to use.

## How To Use (Command Line Only)

If using compiled executeable file, cd to the directory with the slicer executeable<br>
and replace all `cargo run --release --` with `./slicer` when running the commands<br>

If any of the folder/file paths containes spaces make sure to put the paths in double qoutes `"./path/to my/cool pack"`

``cargo run --release -- <input dir or zip> <output dir> [<leftover dir>]`` (``leftover dir`` is optional location that will be filled with copies of source images with added hightlights for no longer needed areas).

## Example Commands
``cargo run --release -- ./My-Cool-Pack ./output``<br>
``cargo run --release -- ./My-Cool-Pack ./output ./leftovers``<br>
``cargo run --release -- ./My-Cool-Pack ./My-Cool-Pack``<br>
``cargo run --release -- ./My-Cool-Pack ./My-Cool-Pack ./leftovers``<br>

Output directory can't be the same as the input if the input is a zip.<br>
``cargo run --release -- ./My-Cool-Pack.zip ./output ./leftovers``
