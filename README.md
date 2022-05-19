<br/>
    <img alt="Hair Banner" height="200" src="https://raw.githubusercontent.com/secondary-smiles/hair/master/social/Hair-Banner.png" />
</br>

# Hair

<!-- SHIELDS BEGIN -->
![GitHub all releases](https://img.shields.io/github/downloads/secondary-smiles/hair/total?logo=GitHub&style=flat-square)
![GitHub issues](https://img.shields.io/github/issues/secondary-smiles/hair?logo=GitHub&style=flat-square)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/secondary-smiles/hair?style=flat-square)
![GitHub Repo stars](https://img.shields.io/github/stars/secondary-smiles/hair?color=yellow&logo=GitHub&style=flat-square)
![GitHub license](https://img.shields.io/github/license/secondary-smiles/hair?style=flat-square)
<!-- SHIELDS END -->

**Hair is a stupid, simple, and small HTTP command-line client made in Rust.**

## Installation
``` bash
$ git clone https://github.com/secondary-smiles/hair.git
$ cd hair
$ make
$ make install
```
### A small guide to the Makefile
`$ make` -- compiles the program, writes log files and backs up existing log files

`$ make clean` -- backs up the log files and cleans everything else up

`$ make install` -- installs the program to /usr/local/bin

`$ make uninstall` -- removes the program from /usr/local/bin

`$ make reset` -- removes the log files and the program from /usr/local/bin

## Usage
```bash
$ hair <METHOD> <URL>
```

In its current version hair has only two options: `<METHOD>` and `<URL>`.

`METHOD` can be GET, POST, PUT, DELETE, etc. Right now there is no checking on this so honestly anything will be sent as typed.

`URL` is the url you want to send the request to. It can be passed as it would be in a browser, it's automatically parsed and split.

**Note that `<METHOD>` and `<URL>` are reorderable**

### Example:
```bash
$ hair GET httpbin.org/get
```

## Contributing
Contributions are always welcome (though I suspect I will be the only one working on this project :P). Feel free to open an issue or even create a pull request on github. (Please make sure it compiles before you submit a PR).

## TODO
- [ ] Add `-` commands to cli (like `-v` for version)
- [X] Make commands reorderable
- [ ] Add to package managers like `brew` and `cargo` (low priority while in alpha)
- [ ] Add support for HTTP 1.1+ and HTTPS
- [ ] Autocomplete
- [ ] Colored output
- [ ] Graceful error handling
- [ ] Create a website for hair
- [ ] Add many many features like output files, file uploads, color themes, etc.
- [ ] Add optional auto redirect for 301, 302, etc.
- [ ] Be even more awesome (general improvement)
