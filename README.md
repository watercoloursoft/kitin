# Kitin
Kitin is a project management tool for github repositories, inspired by https://github.com/mateodelnorte/meta.

Kitin is built in [zig](https://ziglang.org)

## Getting Started
To use Kitin you can either use binaries on the github releases or [compile](#compile) it yourself

## Compile
To compile Kitin you need:
- zig (obtained [here](https://ziglang.org/download/), I'm using version `x86_64-0.10.0-dev.2751+08459ff1c`)

Then it's as simple as
```sh
cd kitin
zig build
```
After the zig compiler runs, an executable file for your platform should be at `zig-out/kitin`. You can customise where the file is output by using `-p your_directory/filename`