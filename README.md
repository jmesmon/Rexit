<div align="center">

# Rexit

Rexit - Liberate your Reddit Chats. This tool will export your Reddit chats into a plethora of formats

![version](https://img.shields.io/github/v/tag/mpult/rexit?color=orange)
![license](https://img.shields.io/github/license/mpult/rexit?color=blue)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/mpult/rexit?color=red)
[![Ubuntu-latest](https://github.com/MPult/Rexit/actions/workflows/Ubuntu-latest.yml/badge.svg)](https://github.com/MPult/Rexit/actions/workflows/Ubuntu-latest.yml)

</div>

Tool to export Reddit chats into a variety of open formats (CSV, JSON, TXT).

```
Export your Reddit Chats

Usage: rexit.exe [OPTIONS] --formats <FORMATS>

Options:
  -f, --formats <FORMATS>  The formats to export to. Options: csv,json,txt
  -t, --token              To use the bearer token flow, instead of username and password
      --debug              Allow debugging of Rexit
  -i, --images             Output images too (outputs to images folder)
  -o, --out <OUT>          What folder to output to [default: ./out]
  -h, --help               Print help
  -V, --version            Print version
```

## Usage

Currently, you need to specify the formats, and it will ask for the username and password (or bearer token with that auth flow).

```bash
$ rexit --formats csv,json,txt --images
> Your Reddit Username: <USERNAME>
> Your Reddit Password: <PASSWORD>
```
It will save the files to the current directory. For CSV and TXT it is split by room. If an image (.jpg, .gif, .png, etc.) was sent the filename will be displayed as the message content, along with the prefix `FILE`. 

## Installation
You can use the files provided in the releases' page of this repository, install via cargo or brew or build from source.

### Manual Install

1. Download the build for your system (Windows or arm64-darwin)
2. Use the terminal run Rexit with the arguments you want. (See Usage for details)

### Cargo Install
```BASH
$ cargo install rexit
```
### Brew Install
To use brew you need to add my tap
```BASH
$ brew tap mpult/mpult
```
Then install Rexit
```BASH
$ brew install rexit
```
### Building from source
1. Install rust
2. Clone the repository
3. Run:
```BASH
$ cargo install --file .
```
## Contributing
To keep the docs focused on the user experience the contributing and technical docs were implemented through cargo doc.

To access these:
```bash
$ cargo doc --open
```

In general all contributions are welcome. I would appreciate if you'd create an issue beforehand, in order for me to plan things out nicely.
## License
[GNU General Public License, Version 3](./LICENSE)
