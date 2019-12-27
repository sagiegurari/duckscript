# duckscript

| duckscript  | SDK     | CLI         |
| ----------- | ------- | ----------- |
| [![crates.io](https://img.shields.io/crates/v/duckscript.svg)](https://crates.io/crates/duckscript) | [![crates.io](https://img.shields.io/crates/v/duckscriptsdk.svg)](https://crates.io/crates/duckscriptsdk) | [![crates.io](https://img.shields.io/crates/v/duckscript_cli.svg)](https://crates.io/crates/duckscript_cli) |
| [![downloads](https://img.shields.io/crates/d/duckscript.svg)](https://crates.io/crates/duckscript) | [![downloads](https://img.shields.io/crates/d/duckscriptsdk.svg)](https://crates.io/crates/duckscriptsdk) | [![downloads](https://img.shields.io/crates/d/duckscript_cli.svg)](https://crates.io/crates/duckscript_cli) |

[![Build Status](https://travis-ci.org/sagiegurari/duckscript.svg?branch=master)](http://travis-ci.org/sagiegurari/duckscript)
[![Build status](https://ci.appveyor.com/api/projects/status/github/sagiegurari/duckscript?branch=master&svg=true)](https://ci.appveyor.com/project/sagiegurari/duckscript)
[![codecov](https://codecov.io/gh/sagiegurari/duckscript/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/duckscript)
[![license](https://img.shields.io/crates/l/duckscript.svg)](https://github.com/sagiegurari/duckscript/blob/master/LICENSE)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Simple, extendable and embeddable scripting language.

* [Overview](#overview)
    * [Language Goals](#lang-goals)
* [Installation](#installation)
    * [Binary Release](#installation-binary-release)
* [Duckscript Tutorial](#tutorial)
    * [Hello World Example](#tutorial-hello-world)
    * [Commands](#tutorial-commands)
        * [Passing Arguments](#tutorial-commands-passing-arguments)
        * [Storing Output](#tutorial-commands-storing-output)
        * [Using Variables](#tutorial-commands-using-variables)
    * [Labels](#tutorial-labels)
    * [Comments](#tutorial-comments)
    * [Pre Processing](#tutorial-pre-processing)
        * [Including Files](#tutorial-pre-processing-including-files)
        * [Printout](#tutorial-pre-processing-printout)
    * [Standard API](#tutorial-standard-api)
        * [Commands Instead Of Language Features](#tutorial-standard-api-commands-lang-features)
        * [Full SDK Docs](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md)
* [Duckscript Command Implementation Tutorial](#sdk-tutorial)
* [Duckscript Embedding Tutorial](#sdk-tutorial)
* [Contributing](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)
* [Release History](https://github.com/sagiegurari/duckscript/blob/master/CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
The duckscript task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command, script, rust code or other sub tasks to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, generate documentation, run bench tests, run security validations and more, executed by running a single command.

<a name="installation"></a>
## Installation
In order to install, just run the following command

```sh
cargo install --force duckscript
```

This will install duckscript in your ~/.cargo/bin.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.<br>
<br>
You will have two executables available: *duckscript* and *makers*<br>

* **duckscript** - This is a cargo plugin invoked using ```cargo make ...```
* **makers** - A standalone executable which provides same features and cli arguments as duckscript but is invoked directly and not as a cargo plugin.

<a name="installation-binary-release"></a>
### Binary Release
Binary releases are available in the [github releases page](https://github.com/sagiegurari/duckscript/releases).<br>
The following binaries are available for each release:

* x86_64-unknown-linux-musl
* x86_64-apple-darwin
* x86_64-pc-windows-msvc

Linux builds for arm are available on [bintray](https://bintray.com/sagiegurari/duckscript/linux)

## Contributing
See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/duckscript/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
