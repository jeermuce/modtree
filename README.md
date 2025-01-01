# ModTree

ModTree is a Rust utility that helps maintain module files in Rust projects by automatically creating and updating mod files in directory trees. It uses the newer style of mod files, where mod files are named after their directory and exist in the parent directory.

## Features

- ✅ **Yes**:
  - Creating mod files for new Rust source files
  - Handling nested directories
    - Will ignore directories that contain no Rust source files
  - Adding appropriate `mod` statements for new files in existing mod directories

- 🔄 **Kinda**:
  - Renaming files/directories (creates new mod entries but doesn't remove old ones)
  - Moving files/directories (creates new mod entries but doesn't remove old ones)

- ❌ **No**:
  - Old-style `mod.rs` files
  - Modifying files that are not modules (e.g. `lib.rs`, `main.rs`)
  - Removing mod statements or files
  
## Installation

```
cargo install --git https://github.com/jeermuce/modtree
```

## Usage

Basic usage with default settings:
```
modtree
```

This will process the default `./src` directory.

Specify a custom source directory:
```
modtree -s path/to/source
```
or
```
modtree --source path/to/source
```

I recommend using watchexec to automatically run ModTree when files change, I use it like this:
```
watchexec --timings --color=always --fs-events=create,remove,rename --delay-run=100ms -w ./src -- modtree -s ./src
``` 

- `--timings` to see how long it takes to run, 'cause numbers are fun.
- `--color=always` to get colored output all the time
- `--fs-events=create,remove,rename` so it only runs whene it has things to do because:  `remove mod statement -> modtree puts it back -> remove file -> remove statement -> modtree runs again` and that's annoying
- `--delay-run=100ms` to wait for the filesystem to settle before running
- `-w ./src` to watch the `./src` directory
- `-- modtree -s ./src` to run ModTree with the specified source directory, I like to be explicit

## Output

ModTree displays a colored tree structure of your Rust source files:
- 🟨 Regular files are shown in yellow
- 🟧 Updated files are marked with "[Updated]"
- 🟩 Created files are marked with "[Created]"

Example output:
```
./src
├── main.rs
├── utils.rs
└── arguments
    ├── args.rs [Created]
    └── parse_path.rs [Updated]
```

## Dependencies

- `anyhow` (1.0.95) - For error handling
- `clap` (4.5.23) - For command-line argument parsing
- `colored` (2.2.0) - For colorized output

## Building from Source

1. Clone the repository:
```
git clone https://github.com/jeermuce/modtree
```

2. Build the project:
```
cd modtree
cargo build --release
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
Also feel free to offer constructive criticism, advice, suggestions, etc. I'm always looking to improve.

## Limitations

- Does not support the older style of mod files (mod.rs in directory)
- Cannot modify files that do not represent modules (e.g. `lib.rs`, `main.rs`)
- Does not handle removal of files or mod statements
- Partial support for file/directory renaming and moving operations
- Occassionally puts mod statements twice in the same file, I haven't figured out why yet, you can just delete the duplicate for now

## Future Plans

I want to add these at some point, feel free to contribute if you want to see them sooner:

- [ ] watchmode so I don't have to use watchexec
    - `-w` and `--watch` flags to enable this, default to not watching
- [ ] Support for the older style of mod files (mod.rs in directory)
    - `-o` and `--old-style` flags to enable this, default to the newer style and `-n` and `--new-style` flags to be explicit about the newer style
- [ ] Full support for renaming/deleting files/directories
    - `-r` and `--remove` flags to enable this, default to not removing anything
- [ ] Support for modifying files that are not modules (e.g. `lib.rs`, `main.rs`)
    - `-m` and `--main` flags to enable this, default to not modifying these files
- [ ] Control over the output format
    - `-f` and `--format` flags to enable this, default to the current format
- [ ] Control over the depth of the output
    - `-d` and `--depth` flags to enable this, default to showing the full tree



## Motivation

I'm just lazy enough to spend several hours writing a program that will save me a few seconds every time I create a new file in a Rust project. I also wanted to learn more about Rust and how to write a CLI program. I'm happy with the result so far, I hope you find it useful too.

---

For bug reports and feature requests, please open an issue on the [GitHub repository](https://github.com/jeermuce/modtree/issues).