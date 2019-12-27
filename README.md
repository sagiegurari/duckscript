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
Duckscript is a simple, extendable and embeddable scripting language.<br>
The language itself has only few rules and most common language features are implemented as commands rather than part of the language itself.

<a name="lang-goals"></a>
### Language Goals
Duckscript scripting language goals are:

* Simple - This is probably the simplest language you will ever see.
* Extendable - Instead of having common features such as functions and conditional blocks be a part of the language, they are actually part of the API. So they can easily be replaced/modified or you can add more 'feature' like commands on your own.
* Embeddable - One of the main purposes of this language is to allow other libraries/executables/apps have scripting capability by embedding duckscript. Embedding is easy and requires only few lines of code.

<a name="installation"></a>
## Installation
If you have [rust](https://www.rust-lang.org/), just run the following command

```sh
cargo install --force duckscript_cli
```

This will install duckscript script runner, the standard duckscript SDK and the duckscript CLI.<br>
You should then have a **duckscript** executable in your ~/.cargo/bin directory.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.

<a name="installation-binary-release"></a>
### Binary Release
Binary releases are available in the [github releases page](https://github.com/sagiegurari/duckscript/releases).<br>
The following binaries are available for each release:

* x86_64-unknown-linux-musl
* x86_64-apple-darwin
* x86_64-pc-windows-msvc

<a name="tutorial"></a>
## Duckscript Tutorial
The following sections will teach you how to write and run duck scripts.

<a name="tutorial-hello-world"></a>
### Hello World Example
Let's take a really simple example (all examples are located in the [examples](https://github.com/sagiegurari/duckscript/tree/master/examples) directory:

```sh
# print the text "hello world"
echo hello world
```

Running this script is done using the **duckscript** executable as follows:

```sh
duckscript ./examples/hello_world.ds
```

We will understand more and breakdown this down in the following sections.

<a name="tutorial-commands"></a>
### Commands
Commands are the basis of everything in duckscript.<br>
Commands may execute some action (like printing "hello world" to the console) or serve as flow control (such as functions or if/else conditions).<br>
In order to invoke an action, simply write the action name:

```sh
echo
```

The basic syntax of a command line is:

```
[:label] [output variable =] [command [arguments]]
```

<a name="tutorial-commands-passing-arguments"></a>
#### Passing Arguments
Commands may accept arguments, for example the command **echo** may accept any number of arguments and it will print all of them.<br>
Arguments are separated with the space character.<br>
So in the example:

```sh
# print the text "hello world"
echo hello world
```

The **echo** command got 2 arguments: "hello" and "world".<br>
If your argument contains a space, you can wrap the entire argument with the ```"``` character as follows:

```sh
# print the text "hello world"
echo "hello world"
```

In which case the **echo** command got only one argument: "hello world" and prints it.<br>
You can escape the ```"``` character using the ```"\"``` character, for example:

```sh
# print the text 'hello "world"'
echo "hello \"world\""
```

In the above example, the **echo** command got one argument: 'hello "world"' and prints it.<br>
The ```"\"``` is also used to escape the following:

* \n - End of line
* \r - Carriage return
* \t - Tab character

<a name="tutorial-commands-storing-output"></a>
#### Storing Output
Commands may return an output which can be stored in a variable.<br>
Variables in duckscript have no strict type.<br>
In the following example, the **set** command takes one argument and stores it in the **out** variable.

```sh
out = set "hello world"
```

Duckscript has only global scope, so once you have stored a value in a variable, you may use it anywhere in your script.

<a name="tutorial-commands-using-variables"></a>
#### Using Variables
Stored variables can be later on used as arguments for other commands.<br>
In order to use a variable, we need to wrap it as follows: ```${variable}```.<br>
<br>
The following example uses the **set** command to store a value in the **out** variable and then prints it:

```sh
out = set "hello world"

# This will print: "The out variable holds the value: hello world"
echo The out variable holds the value: ${out}

# This will print: "To use the out variable just write: ${out}"
echo To use the out variable just write: \${out}
```

In this example, although **out** holds the value **hello world** which contains a space, it is still considered as a single input argument to the **echo** command.<br>
In the second echo command we prevented the variable name from being replaced by escaping it using the ```\``` character.

<a name="tutorial-labels"></a>
### Labels
Labels are simple textual names you can give to a specific line.<br>
Commands like **goto** can then be used to make the script execution jump from its current position to the label position.<br>
For example:

```sh
goto :good

echo error!!!!

:good echo yay!!!!
```

<a name="tutorial-comments"></a>
### Comments
Comments are not executed and are simply in the code for documentation purposes.<br>
A document line must start with the ```#``` character.<br>
You can also have a comment after the command and the command will ignore it.<br>
For example:

```sh
# This is just a comment

echo This will print # But this will not
```

## Contributing
See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/duckscript/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
