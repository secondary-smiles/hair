<br/>
    <img alt="Hair Banner" height="200" src="https://raw.githubusercontent.com/secondary-smiles/hair/master/front/Hair-Banner.png" />
</br>

# Hair

<!-- SHIELDS BEGIN -->
![GitHub all releases](https://img.shields.io/github/downloads/secondary-smiles/hair/total?logo=GitHub&style=flat-square)
![GitHub issues](https://img.shields.io/github/issues/secondary-smiles/hair?logo=GitHub&style=flat-square)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/secondary-smiles/hair?style=flat-square)
![GitHub Repo stars](https://img.shields.io/github/stars/secondary-smiles/hair?color=yellow&logo=GitHub&style=flat-square)
![GitHub license](https://img.shields.io/github/license/secondary-smiles/hair?style=flat-square)
<!-- SHIELDS END -->

Hair is a stupid, simple, and small HTTP command-line client made in Rust.

## Usage
In its current version hair has very strict syntax rules. Note that this will change in the future.

```bash
$ hair <METHOD> <URL>
```
`METHOD` can be GET, POST, PUT, DELETE, etc. Right now there is no checking on this so honestly anything will be sent as typed.

`URL` is the url you want to send the request to. It can be passed as it would be in a browser, it's automatically parsed and split.

## Contributing
Contributions are always welcome (though I suspect I will be the only one working on this project :P). Feel free to open an issue or even create a pull request on github. (Please make sure it compiles before you submit a PR).

## TODO
- [ ] Add `-` commands to cli (like `-v` for version)
- [ ] Make `-` commands reorderable
- [ ] Add to package managers like `brew` and `cargo`
- [ ] Add support for HTTP 1.1+ and HTTPS
- [ ] Autocomplete
- [ ] Colored output
- [ ] Graceful error handling
- [ ] Create a website for hair
- [ ] Add many many features like output files, file uploads, color themes, etc.
- [ ] Add optional auto redirect for 301, 302, etc.
- [ ] Be even more awesome (general improvement)
