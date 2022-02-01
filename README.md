# slicer

## About This Project

This is a Rust port of Mojang's [slicer](https://github.com/Mojang/slicer) program that is written in Java.

## How To Use (Command Line Only)

``cargo run --release -- <input dir or zip> <output dir> [<leftover dir>]`` (``leftover dir`` is optional location that will be filled with copies of source images with added hightlights for no longer needed areas).

## Example Commands
``cargo run --release -- ./My-Cool-Pack ./output``<br>
``cargo run --release -- ./My-Cool-Pack ./output ./leftovers``<br>
``cargo run --release -- ./My-Cool-Pack ./My-Cool-Pack``<br>
``cargo run --release -- ./My-Cool-Pack ./My-Cool-Pack ./leftovers``<br><br>
Output directory can't be the same as the input if the input is a zip.<br>
``cargo run --release -- ./My-Cool-Pack.zip ./output ./leftovers``
