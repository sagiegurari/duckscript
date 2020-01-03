# Table of Contents
* [sdk::Alias (alias)](#sdk__Alias)
* [sdk::Array (array)](#sdk__Array)
* [sdk::Echo (echo)](#sdk__Echo)
* [sdk::Eval (eval)](#sdk__Eval)
* [sdk::ForIn (for)](#sdk__ForIn)
* [sdk::Function (function, fn)](#sdk__Function)
* [sdk::GoTo (goto)](#sdk__GoTo)
* [sdk::If (if)](#sdk__If)
* [sdk::Not (not)](#sdk__Not)
* [sdk::Release (release)](#sdk__Release)
* [sdk::Set (set)](#sdk__Set)
* [sdk::ShowCommandDocumentation (man)](#sdk__ShowCommandDocumentation)
* [sdk::Unalias (unalias)](#sdk__Unalias)
* [sdk::env::GetVar (get_env)](#sdk__env__GetVar)
* [sdk::env::PrintCurrentDirectory (pwd)](#sdk__env__PrintCurrentDirectory)
* [sdk::env::SetCurrentDirectory (cd, set_current_dir)](#sdk__env__SetCurrentDirectory)
* [sdk::env::SetVar (set_env)](#sdk__env__SetVar)
* [sdk::env::UnsetVar (unset_env)](#sdk__env__UnsetVar)
* [sdk::fs::CopyPath (cp)](#sdk__fs__CopyPath)
* [sdk::fs::CreateDirectory (mkdir)](#sdk__fs__CreateDirectory)
* [sdk::fs::CreateEmptyFile (touch)](#sdk__fs__CreateEmptyFile)
* [sdk::fs::DeleteEmptyDirectory (rmdir)](#sdk__fs__DeleteEmptyDirectory)
* [sdk::fs::DeletePath (rm)](#sdk__fs__DeletePath)
* [sdk::fs::GetCanonicalPath (canonicalize)](#sdk__fs__GetCanonicalPath)
* [sdk::fs::GetFileName (basename)](#sdk__fs__GetFileName)
* [sdk::fs::GetParentDirectory (dirname)](#sdk__fs__GetParentDirectory)
* [sdk::fs::List (ls)](#sdk__fs__List)
* [sdk::fs::MovePath (mv)](#sdk__fs__MovePath)
* [sdk::fs::Print (cat)](#sdk__fs__Print)
* [sdk::fs::Read (readfile)](#sdk__fs__Read)
* [sdk::fs::Write (writefile)](#sdk__fs__Write)
* [sdk::math::Calc (calc)](#sdk__math__Calc)
* [sdk::process::Execute (exec)](#sdk__process__Execute)
* [sdk::process::Exit (exit)](#sdk__process__Exit)
* [sdk::test::Assert (assert)](#sdk__test__Assert)
* [sdk::test::AssertEquals (assert_eq)](#sdk__test__AssertEquals)
* [sdk::test::AssertFail (assert_fail)](#sdk__test__AssertFail)
* [sdk::thread::Sleep (sleep)](#sdk__thread__Sleep)


<a name="sdk__Alias"></a>
## sdk::Alias
```sh
var = alias command arguments
```

This command enables to define new commands with default arguments.<br>
The new alias can be invoked with additional arguments that will be appended to the default set.

#### Parameters

Any number of arguments which will be added to the already defined arguments set during the aliasing.

#### Return Value

**true** if the alias was created, else **false**.

#### Examples

```sh
# This example creates a new **my_echo** alias that will print the prefix before the requested arguments.
created = alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
created = my_echo hello world
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

```sh
# Print multiple arguments:
echo hello world

# Print multiple spaces between words
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

```sh
# Simple example iteration over the list of letters:
args = array a b c

for arg in args
    echo current arg is: ${arg}
end_for

release args

# Example nested loops:
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

```sh
# Simple example of a function definition which echo 'hello world' and exists.

# function start
function hello_world
    echo hello world
end_function

# function invocation
hello_world

# Example of calling a function and returning a value
function get_hello_world
    return "hello world"
end_function

# function invocation
text = get_hello_world

# this will print "hello world"
echo ${text}

# Example of passing arguments
function print_input
    # $1 is set with the value 'hello'
    # $2 is set with the value 'world'
    echo ${1} ${2}
end_function

print_input hello world

# Functions can call other functions
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

```sh
# Simple example of an if statement that evaluates the argument value as true and echos "in if"
if true
    echo in if
end_if

# Example of using **not** command to reverse the output value
if not false
    echo in if
end_if

# Example of an if statement that evaluates the command as true and echos "in if"
if set true
    echo in if
end_if

# Example of if condition returning a falsy result and navigation goes to the else block which echos "in else"
if set false
    echo should not be here
else
    echo in else
end_if

# Example of if condition returning a falsy result and navigation goes to the elseif block has a truthy condition
if set false
    echo should not be here
elseif set true
    echo in else if
else
    echo should not be here
end_if

# Nested if example:
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

```sh
# Simple example of converting true/false values
is_false = not true
echo is false: ${is_false}

is_true = not false
echo is true: ${is_true}

# Example of converting command output value
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

```sh
# Return simple 'hello' text value
var = set hello

# Return expanded value: 'home: ....'
var = set "home: ${HOME}"
```


#### Aliases:
set

<a name="sdk__ShowCommandDocumentation"></a>
## sdk::ShowCommandDocumentation
```sh
var = man command
```

Prints and returns the help documentation of the provided command.

#### Parameters

The command name.

#### Return Value

The help documentation or if not found, none.

#### Examples

```sh
man set
```


#### Aliases:
man

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

<a name="sdk__env__GetVar"></a>
## sdk::env::GetVar
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

```sh
# Print the current directory:
pwd

# Print and also store the current directory:
directory = pwd
```


#### Aliases:
pwd

<a name="sdk__env__SetCurrentDirectory"></a>
## sdk::env::SetCurrentDirectory
```sh
cd path
```

Sets the current directory based on the input path.<br>
If no path is provided, it will default to the user home directory.<br>
If the path does not exist, it will return none.

#### Parameters

The new current directory.

#### Return Value

The new current directory or none in case of any error such as target directory not found.

#### Examples

```sh
# Move to user home directory and store the path in the home variable
home = cd

# Move to the requested directory
cd ./scripts
```


#### Aliases:
cd, set_current_dir

<a name="sdk__env__SetVar"></a>
## sdk::env::SetVar
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

<a name="sdk__env__UnsetVar"></a>
## sdk::env::UnsetVar
```sh
unset_env key
```

Removes the environment variable defined by the provided key.

#### Parameters

The name of the environment variable to remove

#### Return Value

None

#### Examples

```sh
unset_env HOME
```


#### Aliases:
unset_env

<a name="sdk__fs__CopyPath"></a>
## sdk::fs::CopyPath
```sh
var = cp source target
```

This command copies the requested file or directory to the target location.<br>
If the source directory is not empty, its entire contents will be copied as well.

#### Parameters

* The source path to copy
* The target path

#### Return Value

**true** if the path was copied.

#### Examples

```sh
# copy a single file
copied = cp ./file1.txt ./file2.txt

# copy a directory
copied = cp ./source ./target
```


#### Aliases:
cp

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

<a name="sdk__fs__CreateEmptyFile"></a>
## sdk::fs::CreateEmptyFile
```sh
var = touch file
```

This command will create an empty file and return true/false if the file exists.<br>
If file exits, it will not be modified.

#### Parameters

The file path.

#### Return Value

If the file exists after the command, it will return true.<br>
In case of any error, it will return false.

#### Examples

```sh
exists = touch ./dir/file.txt
```


#### Aliases:
touch

<a name="sdk__fs__DeleteEmptyDirectory"></a>
## sdk::fs::DeleteEmptyDirectory
```sh
var = rmdir path
```

This command delete the requested empty directory and returns true if successful.<br>
If the path leads to a file or a directory which is not empty, this command will fail.

#### Parameters

A single parameter holding the directory path.

#### Return Value

**true** if the directory was deleted.

#### Examples

```sh
deleted = rmdir ./mydir
```


#### Aliases:
rmdir

<a name="sdk__fs__DeletePath"></a>
## sdk::fs::DeletePath
```sh
var = rm [-r] path
```

This command delete the requested file, empty directory or recursively deletes a directory
and all its content (files and sub directories) if the **-r** flag is provided.

#### Parameters

* Optional flags (currently only -r is supported which indicates recursive deletion)
* The path to delete

#### Return Value

**true** if the path was deleted.

#### Examples

```sh
# delete a file or empty directory
deleted = rm ./target

# deletes a directory and all its content
deleted = rm -r ./target
```


#### Aliases:
rm

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

<a name="sdk__fs__List"></a>
## sdk::fs::List
```sh
var = ls [flags] [path]
```

Lists the file/directory contents.<br>
If no path is provided, the current working directory will be used.<br>
The supported flags are:

* -l - Shows extended information

#### Parameters

* Optional flags (currently only -l is supported)
* Optional path (if not provided, current working directory is used)

#### Return Value

**true** is operation was successful.

#### Examples

```sh
# prints current directory content
ls

# prints current directory content
ls .

# prints examples directory content
ls ./examples

# prints examples directory content with extended info
ls -l ./examples

# prints current directory content with extended info
ls -l

# prints file name
ls ./examples/ls.ds

# prints file name with extended info
ls -l ./examples/ls.ds
```


#### Aliases:
ls

<a name="sdk__fs__MovePath"></a>
## sdk::fs::MovePath
```sh
var = mv source target
```

This command moves the requested source path to the target path.

* If the source and target paths define a file, it will move the file as defined.
* If target path is a directory path, it will move the source file/directory into that target directory path.

All missing parent directories in the target path will be created as needed.

#### Parameters

* The source path to copy
* The target path

#### Return Value

**true** if the move was successful.

#### Examples

```sh
# move a single file
moved = mv ./file1.txt ./file2.txt

# move a single file into the target directory
moved = mv ./file1.txt ./somedir

# move entire directory into another directory
moved = mv ./source ./target/subdir
```


#### Aliases:
mv

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

The file content or none if the file does not exist.

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

The file content or none in case file does not exist.

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

<a name="sdk__math__Calc"></a>
## sdk::math::Calc
```sh
var = calc [operation]
```

The calc command accepts multiple arguments which make up a mathematical operation which will be
calculated and its result will be returned.

#### Parameters

Any number of arguments which will construct a line to calculate.

#### Return Value

The result of the mathematical calculation.

#### Examples

```sh
# result is 36
result = calc 1 + 5 * 7
```


#### Aliases:
calc

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

```sh
# Example of running a command and flushing its output to the parent process.
exec echo hello world

# Example of running a command and storing its output.
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

```sh
# exit with code '0'
code = exit

# exit with code '1'
code = exit 1
```


#### Aliases:
exit

<a name="sdk__test__Assert"></a>
## sdk::test::Assert
```sh
assert value [error message]
```

Used to validate the input is truthy.<br>
If the value is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy and will exist with an error.

#### Parameters

* The value to evaluate
* Optional error message

#### Return Value

**true** if truthy.

#### Examples

```sh
# valid conditions
assert ok
assert true
assert yes

value = set "some text"
assert ${value}

# error conditions
assert
assert false
assert 0
assert false "This is my error message"
```


#### Aliases:
assert

<a name="sdk__test__AssertEquals"></a>
## sdk::test::AssertEquals
```sh
assert_eq value1 value2 [error message]
```

Used to validate the input is the same.<br>
If they are not, the command will exist with an error.

#### Parameters

* Two values to evaluate if they are equal
* Optional error message

#### Return Value

**true** if truthy.

#### Examples

```sh
# valid conditions
assert_eq yes yes
assert_eq false false

value = set "some text"
assert_eq ${value} "some text"

# error conditions
assert_eq 1 2
assert_eq 1 2 "This is my error message"
```


#### Aliases:
assert_eq

<a name="sdk__test__AssertFail"></a>
## sdk::test::AssertFail
```sh
assert_fail [error message]
```

This command will exist with an error.<br>
If error message is provided, it will be used as part of the error output.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
assert_fail

assert_fail "This is my error message"
```


#### Aliases:
assert_fail

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

```sh
# will sleep for 10 milliseconds
time = sleep 10
echo Waited for ${time} milliseconds.
```


#### Aliases:
sleep

### License
Developed by Sagie Gur-Ari and licensed under the
[Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
