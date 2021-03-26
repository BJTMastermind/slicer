<h1 align="center">Slicer and Merger</h1>

  This is a Rust fork of Mojang's [slicer](https://github.com/Mojang/slicer) program that is written in Java and extended to add support to reverse the process.                                                                        

## Plan To Add / To Do

- [ ] Merging support
- [ ] Zip file support

## How To Use (Command Line)

``cargo run --release -- <input dir> <output dir> [<leftover dir>]`` (``leftover dir`` is optional location that will be filled with copies of source images with added hightlights for no longer needed areas).

## Minimum Rust Version

* Rust 1.50.0 

## About This Project

This is a Resource pack migration tool for Minecraft: Java Edition 1.14.

