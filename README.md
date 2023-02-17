# fbpconvert

**fbpconvert is a command-line tool for archiving Factorio blueprints.**

---

## Installation

If you have [Rust](https://rustup.rs/) already installed, then run:

```
git clone https://github.com/cruzerngz/fbpconvert.git
cd <path_to_git_repo>
cargo b --release
```

Alternatively, [download a release instead](https://github.com/cruzerngz/fbpconvert/releases/latest)

---

## Usage


```sh
fbpconvert help             # Show help + usage

fbpconvert import clipboard # import blueprint from clipboard
fbpconvert import file      # import blueprint from file
fbpconvert export clipboard # export a JSON tree to clipboard
fbpconvert export file      # export a JSON tree to file
```

---

## Modifications to your blueprints
fbpconvert may modify your blueprint:

- Due to the need to comply with [file naming conventions](http://www.linfo.org/file_name.html), blueprint/book names that contain special characters will have these characters replaced.

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

## Performance
With the upgrade to [rayon](https://github.com/rayon-rs/rayon), `fbpconvert` now takes advantage of the fact that blueprint books can be serialized/deserialized in parallel.
I went to find the largest blueprint book online, [BobAAAces' blueprint book](https://factorioprints.com/view/-M09ncl6buKTEBZFonGd) to see how `fbpconvert` performs.


### Benchmark times **`(import, export)`**

| CPU | cores | RAM (GB) | OS | realtime (s) | utime (s) | notes |
| --- | --- | --- | --- | --- | --- | --- |
| i5 6200U | 2 | 4 | [eOS](https://elementary.io/) | `(165, 373)` | `(44, 57)` | lots of IO to/from swap |
| i7 10510U | 4| 16 | WSL | `(15, 20)` | `(29, 20)` | |
| r7 6800HS | 8 | 16 | [Fedora](https://getfedora.org/) | `(16, 23)` | `(25, 23)` | limited to 1.8GHz |
| r7 6800HS | 8 | 16 | [Fedora](https://getfedora.org/) | `(8, 11)` | `(17, 11)` | unlimited (4.78GHz) |
| r9 5900X | 12 | 32 | WSL | `(6, 9)` | `(13, 9)` | zen 3 rocks |

Note: this test blueprint book inflates to ~250MB of JSON files! Most blueprint books won't even go past 1MB in size.

---
