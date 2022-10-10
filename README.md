# treestats [![](https://img.shields.io/crates/v/treestats.svg)](https://crates.io/crates/treestats) [![License](https://img.shields.io/badge/license-ISC-blue.svg)](/LICENSE)

**treestats** is a simple tool to compute the count and the total size of each file extension in a directory tree.

## Usage

```
Usage: treestats analyze <directory>
       treestats -h | --help
       treestats --version

Commands:
    analyze                  Analyze a directory tree and output statistics.

Arguments:
    directory                Root directory of the directory tree to analyze.

Options:
    -h, --help               Show this screen.
    --version                Show version.
```

## Example

```console
$ treestats analyze ~/Music | sort -nrk3 | head -n 3
  5654 files, 25.19 GiB analyzed [00:00:02]
  ogg     2999    15536178965
  mp3     1922    10386533620
  wma     85      394384068
```

The output columns are extension, count, total size (in bytes).
Files that do not have an extension, or the extension of which is not valid UTF-8 are grouped under “other”.

## Compiling

Run `cargo build --release` in your working copy.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/treestats/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/treestats/issues).

## License

treestats is copyright (C) 2022 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.
