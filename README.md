# fbpconvert

**fbpconvert is a command-line tool for tracking or archiving Factorio blueprints.**

---

## How it works

Blueprint strings are simply [compressed JSON strings](https://wiki.factorio.com/Blueprint_string_format).
By decompressing and separating each blueprint book and its blueprints into a tree-like file structure, changes to each blueprint can be tracked more easily by a tool like Git.

```
bp_dir/
└── [4.0K]  book_top_level
    ├── [ 169]  .book_top_level.json
    ├── [4.0K]  book_mid_level
    │   ├── [ 169]  .book_mid_level.json
    │   ├── [ 451]  infinity pipe.json
    │   └── [ 470]  loader.json
    ├── [ 539]  infinity chest.json
    └── [ 445]  steel chest.json
```

---

## Limitations

There are, however, some limitations of this tool.

- Blueprints/Books must adhere to [file naming conventions](http://www.linfo.org/file_name.html). So slashes and other special characters cannot be used.

- Blueprint book information is stored in a JSON dotfile that matches the name of its parent directory. So avoid naming blueprints starting with a dot.

---

## Get the binary

If you have Rust already installed, then run:

```
git clone https://github.com/cruzerngz/fbpconvert.git
cd <path_to_git_repo>
cargo b --release
```

Alternatively, download a release instead

---

## Usage

Run `fbpconvert help` for more information

### Importing a blueprint string from factorio

`fbpconvert import clipboard` : import directly from your clipboard

`fbpconvert import file` : import blueprint string from a file

### Exporting a JSON tree to a blueprint string

`fbpconvert export clipboard` : export a blueprint file/directory to the clipboard

`fbpconvert export file` : export a blueprint string to a file

---
