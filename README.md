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

## Output

The output consists of TSV with no header and three columns: extension, count, total size (in bytes), separated by single tabs (`\t`).
Files that do not have an extension, or the extension of which is not valid UTF-8 are grouped under the “other” pseudo-extension.

This enables seamless composition with other tools to filter, sort, pretty-print… the output.

## Examples

### Top 3 extensions sorted by decreasing total size

```console
$ treestats analyze ~/Music | sort -nrk3 | head -n 3
  5654 files, 25.19 GiB analyzed [00:00:02]
  ogg	1922	15536178965
  mp3	2999	10386533620
  wma	85	394384068
```

### Top 3 extensions sorted by decreasing file count, pretty-printed in columns

```console
$ treestats analyze ~/Music | sort -nrk2 | head -n 3 | column -t
  5654 files, 25.19 GiB analyzed [00:00:02]
  mp3  2999  10386533620
  ogg  1922  15536178965
  au   128   97610120
```

Same thing with [csview](https://github.com/wfxr/csview):

```console
$ treestats analyze ~/Music | sort -nrk2 | head -n 3 | csview -H -t -s Rounded
  5654 files, 25.19 GiB analyzed [00:00:02]
╭─────┬──────┬─────────────╮
│ mp3 │ 2999 │ 10386533620 │
│ ogg │ 1922 │ 15536178965 │
│ au  │ 128  │ 97610120    │
╰─────┴──────┴─────────────╯
```

### Top 3 extensions with more than 10 files sorted by increasing file count

```console
$ treestats analyze ~/Music | awk '$2 > 10' | sort -nk2 | head -n 3
  5654 files, 25.19 GiB analyzed [00:00:02]
  xm	11	12172128
  mix	12	31213
  rm	19	5305945
```

### Manipulating data on Windows

We can use cmdlets provided by `powershell`/`pwsh`, though we need to specify delimiter and provide header. In this this example we output top 3 extensions sorted by decreasing total size.
Pretty-printing is handled by powershell.

```console
PS> treestats.exe analyze $HOME\Downloads\ | ConvertFrom-Csv -Delimiter `t -Header 'extension', 'count', 'size' | sort {[int]$_.size} -Descending -top 3
  4893 files, 985.43 MiB analyzed [00:00:00]
extension count size
--------- ----- ----
exe       21    257707977
zip       19    158416662
pdf       128   146198528
```

## Compiling

Run `cargo build --release` in your working copy.

## Contributing and reporting bugs

Contributions are welcome through [GitHub pull requests](https://github.com/Arkanosis/treestats/pulls).

Please report bugs and feature requests on [GitHub issues](https://github.com/Arkanosis/treestats/issues).

## License

treestats is copyright (C) 2022 Jérémie Roquet <jroquet@arkanosis.net> and licensed under the ISC license.
