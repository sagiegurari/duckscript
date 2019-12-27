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
        * [!include_files](#tutorial-pre-processing-including-files)
        * [!print](#tutorial-pre-processing-printout)
    * [Standard API](#tutorial-standard-api)
        * [Commands Instead Of Language Features](#tutorial-standard-api-commands-lang-features)
        * [Full SDK Docs](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md)
    * [Final Notes](#tutorial-final-notes)
* [Duckscript Command Implementation Tutorial](#sdk-tutorial)
* [Duckscript Embedding Tutorial](#embed-tutorial)
    * [Setting Up The Context](#embed-tutorial-setup-context)
    * [Running The Script](#embed-tutorial-running)
* [Contributing](#contributing)
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

<a name="tutorial-pre-processing"></a>
### Pre Processing
Pre processing is the phase that duckscript is parsing the script content.<br>
It is possible to run specific commands at that phase to modify the script during the parsing phase.<br>

The basic syntax of a pre processing command line is:

```
!command [arguments]
```

<a name="tutorial-pre-processing-including-files"></a>
#### !include_files
The include_files command enables you to load script files into the position of the pre processor command.<br>
Basically it enables you to include many scripts and generate one bigger script for runtime.<br>
The include files command accepts any number of files and all will be loaded by the order they are defined.<br>
For example:

```sh
# load the hello_world.ds script here
!include_files ./hello_world.ds

# load 2 scripts here. The hello_world.ds is loaded again.
!include_files ./hello_world.ds ./use_variable.ds
```

Important to note that the script paths included are relative to the script file including them and not to the current working directory.

<a name="tutorial-pre-processing-including-files"></a>
#### !print
The print pre processing command allows to print any number of arguments, which could be useful for debugging.<br>
In the following example, although the print command comes after the echo command, it will execute first as it is invoked in the parsing phase and not in the script execution phase which comes later:

```sh
# this will print "hello world during script execution"
echo hello world during script execution

# this will print "hello world during parsing"
!print hello world during parsing
```

<a name="tutorial-standard-api"></a>
### Standard API
Duckscript is split to several modules and while the script runner does not require it, by default it will come with the standard duckscript API called the duckscript SDK.<br>
This SDK holds the most common commands, some which execute actions (such as echo) and some which serve as flow control (such as function).<br>
The SDK enables users to develop their scripts and have a good starting point without the need to develop the commands on their own (as that is a bit more complex).

<a name="tutorial-standard-api-commands-lang-features"></a>
#### Commands Instead Of Language Features
As mentioned before, duckscript is really simple and only has few basic rules.<br>
In order to provide a more richer development experience, common language features such as functions and conditional blocks have been implemented as commands.<br>
This is an example of the [function command](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md#sdk__Function):

```sh
function print_first_and_second_argument
echo ${$1} ${$2}
return printed
end_function

function run_flow
status = print_first_and_second_argument hello world
echo The printout status is: ${status}
end_function

run_flow
```

This example demonstrates how functions as a concept do not need to be part of the language and can be implemented by anyone as a command.<br>
This also means that other developers can replace the function command with their implementation to provide additional/different functionality.

<a name="tutorial-standard-api-full-sdk-docs"></a>
#### Full SDK Docs
The full SDK docs can be found [here](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md)

<a name="tutorial-final-notes"></a>
### Final Notes
That's It!!!!<br>
That is all the language.<br>
Short, simple, only few rules to follow and you mastered duckscript.<br>

If you want to know what more you can do with it, look at the [SDK docs](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md).<br>
If you want to know how to write your own commands or embed the duckscript runtime in your application, continue reading.

<a name="sdk-tutorial"></a>
## Duckscript Command Implementation Tutorial

<a name="embed-tutorial"></a>
## Duckscript Embedding Tutorial
Embedding duckscript is really simple and this is one of the language main goals.<br>
The duckscript cli basically embeds duckscript so you can look at it as a reference, but basically it boils down to really few lines of code:

```rust
let mut context = Context::new();
duckscriptsdk::load(&mut context.commands)?;
runner::run_script_file(file, context)?;
```

That's it!<br>
Unless you want to provide your own custom SDK, prepopulate the runtime context with custom variables/state or
pull information out of the context after invocation than those 3 lines of code is all you need to do.<br>
Let's go over it line by line.<br>

<a name="embed-tutorial-setup-context"></a>
## Setting Up The Context
The context holds the initial known commands, variables and state (internal objects used by commands).<br>
Running the ```Context::new()``` simply creates a new empty context.<br>
You can add to it any command, variable or state object you want before running the scripts.<br>
In our example we load all default standard API commands into the new context via: ```duckscriptsdk::load(&mut context.commands)?;```

<a name="embed-tutorial-running"></a>
## Running The Script
After we have a context setup, we will run the script.<br>
The **runner** enables to run a script text or a script file.<br>
The following public functions are available:

```rust
/// Executes the provided script with the given context
pub fn run_script(text: &str, context: Context) -> Result<Context, ScriptError>;

/// Executes the provided script file with the given context
pub fn run_script_file(file: &str, context: Context) -> Result<Context, ScriptError>;
```

<a name="contributing"></a>
## Contributing
There are many ways to contribute to duckscript, including:

* Writing more commands and adding them to the standard SDK
* Improving existing commands by adding more features
* Improving the documentation
* Opening issues with new feature requests or bugs found
* Spreading the word :)

As for expanding the language, I personally prefer not to make it complex.<br>
Let's try to add more language feature using commands and not changing the language itself.<br>

See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/duckscript/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
