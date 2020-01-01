# Table of Contents
* [sdk::Alias](#sdk__Alias)
* [sdk::Array](#sdk__Array)
* [sdk::Echo](#sdk__Echo)
* [sdk::Eval](#sdk__Eval)
* [sdk::ForIn](#sdk__ForIn)
* [sdk::Function](#sdk__Function)
* [sdk::GoTo](#sdk__GoTo)
* [sdk::If](#sdk__If)
* [sdk::Not](#sdk__Not)
* [sdk::Release](#sdk__Release)
* [sdk::Set](#sdk__Set)
* [sdk::Unalias](#sdk__Unalias)
* [sdk::env::Get](#sdk__env__Get)
* [sdk::env::PrintCurrentDirectory](#sdk__env__PrintCurrentDirectory)
* [sdk::env::Set](#sdk__env__Set)
* [sdk::env::SetCurrentDirectory](#sdk__env__SetCurrentDirectory)
* [sdk::fs::CreateDirectory](#sdk__fs__CreateDirectory)
* [sdk::fs::GetCanonicalPath](#sdk__fs__GetCanonicalPath)
* [sdk::fs::GetFileName](#sdk__fs__GetFileName)
* [sdk::fs::GetParentDirectory](#sdk__fs__GetParentDirectory)
* [sdk::fs::Print](#sdk__fs__Print)
* [sdk::fs::Read](#sdk__fs__Read)
* [sdk::fs::Write](#sdk__fs__Write)
* [sdk::process::Execute](#sdk__process__Execute)
* [sdk::process::Exit](#sdk__process__Exit)
* [sdk::thread::Sleep](#sdk__thread__Sleep)


<a name="sdk__Alias"></a>
## sdk::Alias
```sh
alias command arguments
```

This command enables to define new commands with default arguments.<br>
The new alias can be invoked with additional arguments that will be appended to the default set.

#### Parameters

Any number of arguments which will be added to the already defined arguments set during the aliasing.

#### Return Value

None

#### Examples

This example creates a new **my_echo** alias that will print the prefix before the requested arguments.

```sh
alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
my_echo hello world
```


#### Aliases:
alias

<a name="sdk__Array"></a>
## sdk::Array
```sh
handle = array value1 value2 value3 ...
```

Creates an array from the input arguments and returns a handle to that array.<br>
This handle can be passed to other commands which support arrays using handles.<br>
Once the array is no longer used, it should be released using the **release** command.

#### Parameters

Any number of arguments which will construct the array.

#### Return Value

A handle to the array.

#### Examples

```sh
handle = array ${var} "hello world" 5 ${another_var}

# once done we should release the handle
release ${handle}
```


#### Aliases:
array

<a name="sdk__Echo"></a>
## sdk::Echo
```sh
echo [arg]*
```

The echo command will printout all provided arguments.<br>
After all input is done, an end of line will be printed as well.

#### Parameters

Any number of arguments may be provided and will be printed.

#### Return Value

The amount of arguments printed.

#### Examples

Print multiple arguments:

```sh
echo hello world
```

Print multiple spaces between words

```sh
echo "hello    world"
```


#### Aliases:
echo

<a name="sdk__Eval"></a>
## sdk::Eval
```sh
eval command arguments
```

The eval command enables to run dynamically created commands.<br>
The command and arguments passed can be variables in the form of ${name}.

#### Parameters

Any number of arguments which will construct a line to evaluate and execute.

#### Return Value

The result of the evaluated line.

#### Examples

```sh
command = set echo
eval ${command} hello world
```


#### Aliases:
eval

<a name="sdk__ForIn"></a>
## sdk::ForIn
```sh
args = array a b c
for arg in args
    # commands
end_for
release args
```

The for/in command enables to iterate over an array (see [array command](#sdk__Array)).<br>
The first argument will contain the current iteration value from the array.<br>
Once all values have been read, it will exit the loop.

#### Parameters

* for
  * The variable name which will hold the current iteration value
  * The string "in"
  * The handle to the array of values to iterate
* end_for - no parameters

#### Return Value

None

#### Examples

Simple example iteration over the list of letters:

```sh
args = array a b c

for arg in args
    echo current arg is: ${arg}
end_for

release args
```

Example nested loops:

```sh
range = array 1 2 3
for i in range
    for j in range
        echo i: ${i} j: ${j}
    end_for
end_for
```


#### Aliases:
for

<a name="sdk__Function"></a>
## sdk::Function
```sh
function my_function
# function content
return output
end_function
```

This command provides the function language feature as a set of commands:

* function - Defines a function start block
* end_function - Defines the end of the function block
* return - Allows to exist a function at any point and return an output
* *function name* - Dynamically created commands based on the function name which are used to invoke the function code.

When a function command is detected, it will search for the end_function command that comes after.<br>
That entire block is considered the function code block (functions cannot be nested in outer functions)<br>

In order to invoke the function, simply call the function name with any amount of paramters.<br>
Those parameters will be set as $1, $2, ... and so on.<br>
Since variables are global, it will overwrite any older values stored in those variables.<br>

To exist a function and return a value, simply use the **return** command with the value you want to return.<br>
The variable that was used when the function was originally called, will now store that value.<br>
The return command can be used to exist early without any value.<br>
In case the code reached the **end_function** call, the function will exist but will return not value.

#### Parameters

* function - The function name used later on to invoke the function
* end_function - no parameters
* return - optional single paramter to return as an output of the function call
* *function name* - Any number of arguments which will automatically be set as global variables: $1, $2, ... as so on.

#### Return Value

The function invocation returns the output provided by the return command.

#### Examples

Simple example of a function definition which echo 'hello world' and exists.

```sh
# function start
function hello_world

echo hello world

end_function

# function invocation
hello_world
```

Example of calling a function and returning a value

```sh
function get_hello_world

return "hello world"

end_function

# function invocation
text = get_hello_world

# this will print "hello world"
echo ${text}
```

Example of passing arguments

```sh
function print_input

# $1 is set with the value 'hello'
# $2 is set with the value 'world'
echo ${1} ${2}

end_function

print_input hello world
```

Functions can call other functions

```sh
function get_one
return 1
end_function

function get_number
number = get_one
return ${number}
end_function

output = get_number

# this will print 1
echo ${output}
```


#### Aliases:
function, fn

<a name="sdk__GoTo"></a>
## sdk::GoTo
```sh
goto :label
```

The goto command enables you to jump to any position in the script, if that position has a label value.

#### Parameters

A single valid label value.

#### Return Value

None

#### Examples

```sh
goto :good

echo bad

:good echo good
```


#### Aliases:
goto

<a name="sdk__If"></a>
## sdk::If
```sh
if command|value
    # commands
elseif command|value
    # commands
else
    # commands
end_if
```

This command provides the if/elseif/else condition language feature as a set of commands:

* if - Defines an if condition
* elseif - Defines optional secondary condition blocks
* else - Optinoal fallback block
* end_if - Defines the end of the entire if/else block

if and elseif commands accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command

If the value or the result of the command is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.<br>
In case of falsy value, it will skip to the next elseif/else block.<br>
If a truthy (non falsy) output is found, it will invoke the commands of that code block and ignore all other elseif/else blocks.<br>

if blocks can be nested in other if blocks (see examples).

#### Parameters

* if/elseif - A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.
* else/end_if - no parameters

#### Return Value

None

#### Examples

Simple example of an if statement that evaluates the argument value as true and echos "in if"

```sh
if true
    echo in if
end_if
```

Example of using **not** command to reverse the output value

```sh
if not false
    echo in if
end_if
```

Example of an if statement that evaluates the command as true and echos "in if"

```sh
if set true
    echo in if
end_if
```

Example of if condition returning a falsy result and navigation goes to the else block which echos "in else"

```sh
if set false
    echo should not be here
else
    echo in else
end_if
```

Example of if condition returning a falsy result and navigation goes to the elseif block has a truthy condition

```sh
if set false
    echo should not be here
elseif set true
    echo in else if
else
    echo should not be here
end_if
```

Nested if example:

```sh
if set false
    echo should not be here
elseif set true
    echo in else if but not done yet

    if set true
        echo nested if
    end_if
else
    echo should not be here
end_if
```


#### Aliases:
if

<a name="sdk__Not"></a>
## sdk::Not
```sh
output = not command|value
```

Enables to switch falsy to true and truthy to false.<br>
The **not** commands accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command

If the value or the result of the command is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It will return true, otherwise it will return false.

#### Parameters

A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.

#### Return Value

The switched value of the input.

#### Examples

Simple example of converting true/false values

```sh
is_false = not true
echo is false: ${is_false}

is_true = not false
echo is true: ${is_true}
```

Example of converting command output value

```sh
is_false = not set true
echo is false: ${is_false}

is_true = not set false
echo is true: ${is_true}
```


#### Aliases:
not

<a name="sdk__Release"></a>
## sdk::Release
```sh
release handle
```

Releases an internal handle stored in the runtime memory.<br>
Certain commands (such as **array**) will create a handle and the variable will only hold a reference to that handle.<br>
In order to release those handles once they are no longer needed, the release command should be used.

#### Parameters

The handle name.

#### Return Value

* true - If a handle was found and removed
* false - If no handle was found

#### Examples

```sh
release ${array_handle}
```


#### Aliases:
release

<a name="sdk__Set"></a>
## sdk::Set
```sh
var = set arg
```

The set command will simply return the provided argument and set it to the output variable.

#### Parameters

Only the first argument will be returned.


#### Return Value

The first command argument.

#### Examples

Return a simple text value:

```sh
var = set hello
```

Return an expanded value:

```sh
var = set "home: ${HOME}"
```


#### Aliases:
set

<a name="sdk__Unalias"></a>
## sdk::Unalias
```sh
unalias name
```

Removes previously defined alias and return true/false based if an alias was actually removed.

#### Parameters

The alias name to remove.

#### Return Value

A true/false value in case an alias with the provided name existed.

#### Examples

```sh
alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
my_echo hello world

unalias my_echo

# This will error
echo The script will now error as my_echo is no longer defined
my_echo hello world
```


#### Aliases:
unalias

<a name="sdk__env__Get"></a>
## sdk::env::Get
```sh
var = get_env key
```

Returns the environment variable value for the provided key.

#### Parameters

First argument is the environment variable key.

#### Return Value

The environment variable value.

#### Examples

```sh
home = get_env HOME
```


#### Aliases:
get_env

<a name="sdk__env__PrintCurrentDirectory"></a>
## sdk::env::PrintCurrentDirectory
```sh
var = pwd
```

Prints and also returns the current directory.

#### Parameters

None

#### Return Value

The current directory path.

#### Examples

Print the current directory:

```sh
pwd
```

Print and also store the current directory:

```sh
directory = pwd
```


#### Aliases:
pwd

<a name="sdk__env__Set"></a>
## sdk::env::Set
```sh
set_env key value
```

Sets the environment variable defined by the provided key to the provided value.

#### Parameters

Two arguments are required:

* key - The name of the environment variable to set
* value - The new environment variable value

#### Return Value

None

#### Examples

```sh
set_env HOME /usr/me
```


#### Aliases:
set_env

<a name="sdk__env__SetCurrentDirectory"></a>
## sdk::env::SetCurrentDirectory
```sh
cd path
```

Sets the current directory based on the input path.<br>
If no path is provided, it will default to the user home directory.<br>
If the path does not exist, it will return an error.

#### Parameters

The new current directory.

#### Return Value

The new current directory.

#### Examples

Move to user home directory and store the path in the home variable

```sh
home = cd
```

Move to the requested directory

```sh
cd ./scripts
```


#### Aliases:
cd, set_current_dir

<a name="sdk__fs__CreateDirectory"></a>
## sdk::fs::CreateDirectory
```sh
var = mkdir directory
```

This command will create the requested directory (and needed parent directories) and return true/false if it was successful.

#### Parameters

The directory name to create.

#### Return Value

The operation success value - true if directory exists, else false.

#### Examples

```sh
exists = mkdir ./dir/subdir
```


#### Aliases:
mkdir

<a name="sdk__fs__GetCanonicalPath"></a>
## sdk::fs::GetCanonicalPath
```sh
var = canonicalize path
```

This command will return the c path for the provided input.<br>
In case unable, it will return the original input.

#### Parameters

The file/directory path to canonicalize.

#### Return Value

The canonicalized path, or if unsuccessful, the original path.

#### Examples

```sh
path = canonicalize ./target
```


#### Aliases:
canonicalize

<a name="sdk__fs__GetFileName"></a>
## sdk::fs::GetFileName
```sh
var = basename path
```

This command will return the last path element of the provided path.<br>
If unable, it will return none.

#### Parameters

The path to extract the last element from.

#### Return Value

The last path element or none if unsuccessful.

#### Examples

```sh
file = basename ./dir/file.txt
```


#### Aliases:
basename

<a name="sdk__fs__GetParentDirectory"></a>
## sdk::fs::GetParentDirectory
```sh
var = dirname path
```

This command will return the parent path of the provided path.<br>
If the parent path is empty, it will return none.

#### Parameters

The path to extract the parent path from.

#### Return Value

The parent path or none.

#### Examples

```sh
directory = dirname ./dir/file.txt
```


#### Aliases:
dirname

<a name="sdk__fs__Print"></a>
## sdk::fs::Print
```sh
var = cat file
```

The cat command will print out the requested file.<br>
In addition it will also return the value to the output variable.

#### Parameters

A single parameter holding the file path.

#### Return Value

The file content.

#### Examples

```sh
cat ./docs/sdk.md
```


#### Aliases:
cat

<a name="sdk__fs__Read"></a>
## sdk::fs::Read
```sh
var = readfile file
```

The readfile command will read the requested file and return the value to the output variable.

#### Parameters

A single parameter holding the file path.

#### Return Value

The file content.

#### Examples

```sh
text = readfile ./Cargo.toml
```


#### Aliases:
readfile

<a name="sdk__fs__Write"></a>
## sdk::fs::Write
```sh
result = writefile file text
```

This command enables to write the provided text into the requested file.<br>
It will return true/false value based if it was able to write the text to the file.

#### Parameters

* The target file
* The text content to write

#### Return Value

true/false based if it was able to write the text to the file.

#### Examples

```sh
out = writefile ./target/tests/writefile.txt "line 1\nline 2"
```


#### Aliases:
writefile

<a name="sdk__process__Execute"></a>
## sdk::process::Execute
```sh
exec command [args]*

output = exec command [args]*
stdout = set ${output.stdout}
stderr = set ${output.stderr}
exit_code = set ${output.code}
```

Executes the provided native command and arguments.<br>
If no output variable is set, the command output will be flushed to the main out/err stream of the parent process.<br>
If an output variable is set, it will be used as a base variable name from which the command stout, stderr and exit code information can be pulled from.<br>
The actual output variable name will not be modified, instead new variables will be created using that variable name as a baseline:

* *output*.stdout - Will hold the stdout text content.
* *output*.stderr - Will hold the stderr text content.
* *output*.code - Will hold the process exit code.

#### Parameters

The command to execute and its arguments.

#### Return Value

Optionally a base name to access the process stout, stderr and exit code information.

#### Examples

Example of running a command and flushing its output to the parent process.

```sh
exec echo hello world
```

Example of running a command and storing its output.

```sh
output = exec echo hello world

stdout = set ${output.stdout}
stderr = set ${output.stderr}
exit_code = set ${output.code}

echo stdout: ${stdout}
echo stderr: ${stderr}
echo exit code: ${exit_code}
```


#### Aliases:
exec

<a name="sdk__process__Exit"></a>
## sdk::process::Exit
```sh
code = exit [code]
```

Exits the script with the given code stored in the output variable.

#### Parameters

A positive number as exit code or none for 0.

#### Return Value

The exit code.

#### Examples

Example of exit with code '0'

```sh
code = exit
```

Example of exit with error code '1'

```sh
code = exit 1
```


#### Aliases:
exit

<a name="sdk__thread__Sleep"></a>
## sdk::thread::Sleep
```sh
sleep millies
```

Will cause the script execution to half for the given amount of milliseconds.<br>
The command will also return the amount of milliseconds waited.

#### Parameters

A positive numeric value.

#### Return Value

The amount of milliseconds waited.

#### Examples

Example of sleep for 10 milliseconds"

```sh
time = sleep 10
echo Waited for ${time} milliseconds.
```


#### Aliases:
sleep

### License
Developed by Sagie Gur-Ari and licensed under the
[Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
