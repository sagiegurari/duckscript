
<a name="overview"></a>
## Overview
Duckscript is a simple, extendable and embeddable scripting language.<br>
The language itself has only few rules and most common language features are implemented as commands rather than part of the language itself.

<a name="lang-goals"></a>
### Language Goals
Duckscript scripting language goals are:

* Simple - This is probably the simplest language you will ever see.
* Extendable - Instead of having common features such as functions and conditional blocks be a part of the language, they are actually part of the API. So they can easily be replaced/modified or you can add more 'feature' like commands on your own.
* Embeddable - One of the main purposes of this language is to allow other libraries/executables/apps have scripting capability by embedding duckscript. Embedding is easy (for [rust](https://www.rust-lang.org/)) and requires only few lines of code.

<a name="installation"></a>
## Installation
If you have [rust](https://www.rust-lang.org/), just run the following command

```sh
cargo install --force duckscript_cli
```

This will install duckscript script runner, the standard duckscript SDK and the duckscript CLI.<br>
You should then have a **duck** executable in your ~/.cargo/bin directory.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.

<a name="installation-homebrew"></a>
### Homebrew

```sh
brew install duckscript
```

More details in the [brew page](https://formulae.brew.sh/formula/duckscript)

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
# print the text "Hello World"
echo Hello World
```

Running this script is done using the **duck** executable as follows:

```sh
duck ./examples/hello_world.ds
```

We will understand more and break this down in the following sections.

*Running the duck command without any arguments will open up the repl mode.*

<a name="tutorial-commands"></a>
### Commands
Commands are the basis of everything in duckscript.<br>
Commands may execute some action (like printing "Hello World" to the console) or serve as flow control (such as functions or if/else conditions).<br>
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
# print the text "Hello World"
echo Hello World
```

The **echo** command got 2 arguments: "Hello" and "World".<br>
If your argument contains a space, you can wrap the entire argument with the ```"``` character as follows:

```sh
# print the text "Hello World"
echo "Hello World"
```

In which case the **echo** command got only one argument: "Hello World" and prints it.<br>
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
out = set "Hello World"
```

Duckscript has only global scope, so once you have stored a value in a variable, you may use it anywhere in your script.

<a name="tutorial-commands-using-variables-binding"></a>
#### Using Variables - Binding
Stored variables can be later on used as arguments for other commands.<br>
In order to use a variable, we need to wrap it as follows: ```${variable}```.<br>
<br>
The following example uses the **set** command to store a value in the **out** variable and then prints it:

```sh
out = set "Hello World"

# This will print: "The out variable holds the value: Hello World"
echo The out variable holds the value: ${out}

# This will print: "To use the out variable just write: ${out}"
echo To use the out variable just write: \${out}
```

In this example, although **out** holds the value **Hello World** which contains a space, it is still considered as a single input argument to the **echo** command.<br>
In the second echo command we prevented the variable name from being replaced by escaping it using the ```\``` character.

<a name="tutorial-commands-using-variables-spread-binding"></a>
#### Using Variables - Spread Binding

Spread binding provides a way to convert a variable value into multiple command arguments.<br>
For example:

```sh
out = set "Hello World"
```

The **out** variable holds the value "Hello World".<br>
If we were to create an array from it using the **array** command as follows:

```sh
list = array ${out}
```

The array would be of size 1 and its only entry value would be "Hello World".<br>
So it is the same as if we wrote:

```sh
list = array "Hello World"
```

But what if we want to split the value to multiple parts separated by spaces?<br>
For that we have the spread binding which is defined as follows: ```%{variable}```.<br>
For example:

```sh
list = array %{out}
```

Which would act the same as:

```sh
list = array Hello World
```

And now our array is of size 2 with first entry "Hello" and second entry "World".

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
# this will print "Hello World during script execution"
echo Hello World during script execution

# this will print "Hello World during parsing"
!print Hello World during parsing
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
This is an example of the [function command](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md#std__flowcontrol__Function):

```sh
fn print_first_and_second_argument
    echo ${1} ${2}
    return printed
end

fn run_flow
    status = print_first_and_second_argument Hello World
    echo The printout status is: ${status}
end

run_flow
```

This example demonstrates how functions as a concept do not need to be part of the language and can be implemented by anyone as a command.<br>
This also means that other developers can replace the function command with their implementation to provide additional/different functionality.<br>

Below an example of loops using the [for/in command](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md#std__flowcontrol__ForIn):

```sh
values = range 1 10

for i in ${values}
    for j in ${values}
        echo i: ${i} j: ${j}
    end
end

release ${values}
```

Below an example of [if/else command](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md#std__flowcontrol__If):

```sh
echo Enter Full Name:
name = read

if is_empty ${name}
    echo You did not enter any value
else
    echo Your name is: ${name}
end

value = set false
if ${value}
    echo should not be here
elseif true or false
    echo in else if but not done yet

    value = set true

    if not true and false
        echo nested if

        value = set "some text"

        if starts_with ${value} "some"
            echo after command
        else
            echo should not be here
        end
    end
else
    echo should not be here
end
```

<a name="tutorial-standard-api-full-sdk-docs"></a>
#### Full SDK Docs
The full SDK docs can be found [here](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md)

Keep in mind that the command names (such as **std::Echo**) can be used to invoke the commands, however for simplicity, the documentation
examples uses the alias form of the commands (for example: **echo**).<br>
Each command may have multiple aliases which can be used to invoke it.

<a name="tutorial-final-notes"></a>
### Final Notes
That's It!!!!<br>
That is all the language.<br>
Short, simple, only few rules to follow and you mastered duckscript.<br>

If you want to know what more you can do with it, look at the [SDK docs](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md).<br>
If you want to know how to write your own commands or embed the duckscript runtime in your application, continue reading.

<a name="sdk-tutorial"></a>
## Duckscript Command Implementation Tutorial
Want to write new custom commands so you can use them in your duckscripts? great!<br>
Hopefully the following sections will help you gain the basic knowledge on how to write them.<br>

First of all it is important to understand that there are two types of commands:

* Commands which execute some action like copying files, printing some text to the console or returning an environment variable.
* Commands which provide flow control or some more complex action and require modifying the internal context in runtime.

<a name="sdk-tutorial-standard-commands"></a>
## Standard Commands
Commands are structs that must implement the Command trait.<br>

* They must have a name, which is used to invoke the command.<br>
* They optionally may have aliases which can also be used to invoke the command.<br>
* They should return help documentation in markdown format in order to generate SDK documentation (must for PRs to duckscript official SDK).<br>
* They must implement the **run** function which holds the command logic.<br>

The run function accepts the command arguments (variables already replaced to actual values) and returns the command result.<br>
The command result can be one of the following:

* Continue(Option<String>) - Tells the runner to continue to the next command and optionally set the output variable the given value.
* GoTo(Option<String>, GoToValue) - Tells the runner to jump to the requested line or label and optionally set the output variable the given value.
* Error(String) - Returns the value 'false' and invokes the 'on_error' command if exists with the error message and instruction information.
* Crash(String) - Tells the runner to stop the execution and return the error message.
* Exit(Option<String>) - Tells the runner to stop the execution and optionally set the output variable the given value.

Let's implement a simple **set** command which accepts a single argument and sets the output variable to that value.<br>
And if no argument was provided, return a None which will tell the runner to delete the output variable.<br>
Afterwards the runner should continue to the next line.<br>
So we need to use a Continue(Option<String>) result.<br>
Full example:

```rust
struct SetCommand {}

impl Command for SetCommand {
    fn name(&self) -> String {
        "set".to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let output = if arguments.is_empty() {
            None
        } else {
            Some(arguments[0].clone())
        };

        CommandResult::Continue(output)
    }
}
```

Let's implement a **get_env** command which pulls an environment variable value and sets the output variable with that value.<br>
In case no input was provided we will throw an error, otherwise we will use the first input as the environment variable key.<br>
We will return the value if exists or nothing if it does not.<br>
Full example:

```rust
struct GetEnvCommand {
    package: String,
}

impl Command for GetEnvCommand {
    fn name(&self) -> String {
        "get_env".to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing environment variable name.".to_string())
        } else {
            match env::var(&arguments[0]) {
                Ok(value) => CommandResult::Continue(Some(value)),
                Err(_) => CommandResult::Continue(None),
            }
        }
    }
}
```

You can look at more examples in the duckscript_sdk folder.

<a name="sdk-tutorial-context-commands"></a>
## Context Commands
Context commands are exactly the same as standard commands except that they have access to the runtime context.<br>
Therefore they implement the same Command trait but this time instead of implementing the run function, they need to implement the following:

* requires_context - Must return true
* run_with_context - The same logic you would put in the run function but now you have access to a lot more of the runtime context.

The run_with_context signature is as follows:

```rust
/// Run the instruction with access to the runtime context.
///
/// # Arguments
///
/// * `arguments` - The command arguments array
/// * `state` - Internal state which is only used by commands to store/pull data
/// * `variables` - All script variables
/// * `output_variable` - The output variable name (if defined)
/// * `instructions` - The entire list of instructions which make up the currently running script
/// * `commands` - The currently known commands
/// * `line` - The current instruction line number (global line number after including all scripts into one global script)
/// * `env` - The current runtime env with access to out/err writers, etc...
fn run_with_context(
    &self,
    arguments: Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    output_variable: Option<String>,
    instructions: &Vec<Instruction>,
    commands: &mut Commands,
    line: usize,
    env: &mut Env,
) -> CommandResult;
```

With access to this context you can add/remove/switch commands in runtime, store/pull internal state, add/remove/change variables and so on...

<a name="embed-tutorial"></a>
## Duckscript Embedding Tutorial
Embedding duckscript is really simple and this is one of the language main goals.<br>
The duckscript cli basically embeds duckscript so you can look at it as a reference, but basically it boils down to really few lines of code:

```rust
let mut context = Context::new();
duckscriptsdk::load(&mut context.commands)?;
runner::run_script_file(file, context, None)?;
```

That's it!<br>
Unless you want to provide your own custom SDK, pre populate the runtime context with custom variables/state or
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
pub fn run_script(text: &str, context: Context, env: Option<Env>) -> Result<Context, ScriptError>;

/// Executes the provided script file with the given context
pub fn run_script_file(file: &str, context: Context, env: Option<Env>) -> Result<Context, ScriptError>;
```

<a name="editor-support"></a>
## Editor Support
* Vim: [vim-duckscript](https://github.com/nastevens/vim-duckscript)

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
