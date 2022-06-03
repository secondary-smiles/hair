<br/>
    <a href="https://github.com/secondary-smiles/hair">
    <img alt="Hair Banner" height="200" src="https://raw.githubusercontent.com/secondary-smiles/hair/master/social/Hair-Banner.png" />
    </a>
</br>

# Hair

<!-- SHIELDS BEGIN -->

[![GitHub all releases](https://img.shields.io/github/downloads/secondary-smiles/hair/total?logo=GitHub)](https://github.com/secondary-smiles/hair/releases)
[![GitHub issues](https://img.shields.io/github/issues/secondary-smiles/hair?logo=GitHub)](https://github.com/secondary-smiles/hair/issues)
[![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/secondary-smiles/hair)](https://github.com/secondary-smiles/hair)
[![GitHub Repo stars](https://img.shields.io/github/stars/secondary-smiles/hair?color=yellow&logo=GitHub)](https://github.com/secondary-smiles/hair/stargazers)
[![GitHub license](https://img.shields.io/github/license/secondary-smiles/hair)](https://github.com/secondary-smiles/hair/blob/master/LICENSE.txt)
[![Build status rust](https://github.com/secondary-smiles/hair/actions/workflows/rust.yml/badge.svg)](https://github.com/secondary-smiles/hair/actions/workflows/rust.yml)
[![Build status makefile](https://github.com/secondary-smiles/hair/actions/workflows/makefile.yml/badge.svg)](https://github.com/secondary-smiles/hair/actions/workflows/makefile.yml)

<!-- SHIELDS END -->

**Hair is a stupid, simple, and small HTTP command-line client made in Rust.**

<a href="https://github.com/secondary-smiles/hair">
    <img alt="Hair Demo" width="349" src="https://raw.githubusercontent.com/secondary-smiles/hair/master/social/Hair-Demo.gif" />
</a>

## Installation

```bash
git clone https://github.com/secondary-smiles/hair.git
cd hair
make
make install
```

### A small guide to the Makefile

`make` -- compiles the program, writes log files and backs up existing log files

`make clean` -- backs up the log files and cleans everything else up

`make install` -- installs the program to /usr/local/bin

`make uninstall` -- removes the program from /usr/local/bin

`make reset` -- removes the log files and the program from /usr/local/bin

`make update` -- update from github and install the program using make and make install

## Usage

```bash
hair -h
hair [OPTIONS] [method] URL
```

`method` can be GET, POST, PUT, DELETE, etc. It is the optional HTTP method to use. If not specified, the default method is GET.

`URL` is the url you want to send the request to. It can be passed as it would be in a browser, it's automatically parsed and split.

**Note that `[method]` and `URL` are reorderable**

### Example:

```bash
hair GET httpbin.org/get
```
<sup>Print the server header and response from the url `httpbin.org/get`</sup>

```bash
hair -b httpbin.org/get
```
<sup>Print only the body of the response from the url `httpbin.org/get`</sup>

## Contributing
Contributions are always welcome (though I suspect I will be the only one working on this project :P). Feel free to open an issue or even create a pull request on github.

## TODO

- [x] Add `-` commands to cli (like `-v` for version)
- [x] Make commands reorderable
- [x] Graceful error handling
- [ ] Autocomplete
- [ ] Colored output
- [ ] Create a website for hair
- [ ] Add many many features like output files, file uploads, color themes, etc.
- [ ] Add optional auto redirect for 301, 302, etc.
- [ ] Be even more awesome (general improvement)
- [ ] Add support for HTTP 1.1+ and HTTPS
- [ ] Add to package managers like `brew` and `cargo` (low priority while in alpha)
