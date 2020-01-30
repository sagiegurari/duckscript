# Table of Contents
* [std::Echo (echo)](#std__Echo)
* [std::Eval (eval)](#std__Eval)
* [std::ForIn (for)](#std__ForIn)
* [std::Function (function, fn)](#std__Function)
* [std::GoTo (goto)](#std__GoTo)
* [std::If (if)](#std__If)
* [std::IsDefined (is_defined)](#std__IsDefined)
* [std::Not (not)](#std__Not)
* [std::ReadUserInput (read)](#std__ReadUserInput)
* [std::Release (release)](#std__Release)
* [std::Set (set)](#std__Set)
* [std::ShowCommandDocumentation (man)](#std__ShowCommandDocumentation)
* [std::alias::Set (alias)](#std__alias__Set)
* [std::alias::Unset (unalias)](#std__alias__Unset)
* [std::collections::Array (array)](#std__collections__Array)
* [std::collections::ArrayConcat (array_concat)](#std__collections__ArrayConcat)
* [std::collections::ArrayIsEmpty (array_is_empty)](#std__collections__ArrayIsEmpty)
* [std::collections::ArrayJoin (array_join)](#std__collections__ArrayJoin)
* [std::collections::ArrayLength (array_length, arrlen)](#std__collections__ArrayLength)
* [std::collections::ArrayPop (array_pop)](#std__collections__ArrayPop)
* [std::collections::ArrayPush (array_push)](#std__collections__ArrayPush)
* [std::collections::IsArray (is_array)](#std__collections__IsArray)
* [std::collections::Range (range)](#std__collections__Range)
* [std::collections::ReadProperties (read_properties)](#std__collections__ReadProperties)
* [std::collections::WriteProperties (write_properties)](#std__collections__WriteProperties)
* [std::debug::DuckscriptSDKVersion (duckscript_sdk_version)](#std__debug__DuckscriptSDKVersion)
* [std::debug::DuckscriptVersion (duckscript_version)](#std__debug__DuckscriptVersion)
* [std::debug::DumpInstructions (dump_instructions)](#std__debug__DumpInstructions)
* [std::debug::DumpState (dump_state)](#std__debug__DumpState)
* [std::debug::DumpVariables (dump_variables)](#std__debug__DumpVariables)
* [std::env::GetHomeDirectory (get_home_dir)](#std__env__GetHomeDirectory)
* [std::env::GetVar (get_env)](#std__env__GetVar)
* [std::env::PrintCurrentDirectory (pwd, print_current_directory)](#std__env__PrintCurrentDirectory)
* [std::env::SetCurrentDirectory (cd, set_current_dir, set_current_directory)](#std__env__SetCurrentDirectory)
* [std::env::SetVar (set_env)](#std__env__SetVar)
* [std::env::UnsetVar (unset_env)](#std__env__UnsetVar)
* [std::error::GetLastError (get_last_error)](#std__error__GetLastError)
* [std::error::GetLastErrorLine (get_last_error_line)](#std__error__GetLastErrorLine)
* [std::error::GetLastErrorSource (get_last_error_source)](#std__error__GetLastErrorSource)
* [std::error::SetError (set_error)](#std__error__SetError)
* [std::error::SetExitOnError (exit_on_error, set_exit_on_error)](#std__error__SetExitOnError)
* [std::error::TriggerError (trigger_error)](#std__error__TriggerError)
* [std::fs::Append (appendfile)](#std__fs__Append)
* [std::fs::CopyPath (cp)](#std__fs__CopyPath)
* [std::fs::CreateDirectory (mkdir)](#std__fs__CreateDirectory)
* [std::fs::CreateEmptyFile (touch)](#std__fs__CreateEmptyFile)
* [std::fs::DeleteEmptyDirectory (rmdir)](#std__fs__DeleteEmptyDirectory)
* [std::fs::DeletePath (rm)](#std__fs__DeletePath)
* [std::fs::GetCanonicalPath (canonicalize)](#std__fs__GetCanonicalPath)
* [std::fs::GetFileName (basename)](#std__fs__GetFileName)
* [std::fs::GetParentDirectory (dirname)](#std__fs__GetParentDirectory)
* [std::fs::List (ls)](#std__fs__List)
* [std::fs::MovePath (mv)](#std__fs__MovePath)
* [std::fs::Print (cat)](#std__fs__Print)
* [std::fs::Read (readfile)](#std__fs__Read)
* [std::fs::Write (writefile)](#std__fs__Write)
* [std::math::Calc (calc)](#std__math__Calc)
* [std::math::GreaterThan (greater_than)](#std__math__GreaterThan)
* [std::math::LessThan (less_than)](#std__math__LessThan)
* [std::net::Hostname (hostname)](#std__net__Hostname)
* [std::net::HttpClient (http_client)](#std__net__HttpClient)
* [std::net::WGet (wget)](#std__net__WGet)
* [std::process::Execute (exec)](#std__process__Execute)
* [std::process::Exit (exit, quit, q)](#std__process__Exit)
* [std::process::ProcessID (pid, process_id)](#std__process__ProcessID)
* [std::process::Watchdog (watchdog)](#std__process__Watchdog)
* [std::scope::Clear (clear_scope)](#std__scope__Clear)
* [std::string::Concat (concat)](#std__string__Concat)
* [std::string::Contains (contains)](#std__string__Contains)
* [std::string::EndsWith (ends_with)](#std__string__EndsWith)
* [std::string::Equals (equals, eq)](#std__string__Equals)
* [std::string::IndexOf (indexof)](#std__string__IndexOf)
* [std::string::IsEmpty (is_empty)](#std__string__IsEmpty)
* [std::string::LastIndexOf (last_indexof)](#std__string__LastIndexOf)
* [std::string::Length (length, strlen)](#std__string__Length)
* [std::string::Replace (replace)](#std__string__Replace)
* [std::string::Split (split)](#std__string__Split)
* [std::string::StartsWith (starts_with)](#std__string__StartsWith)
* [std::string::SubString (substring)](#std__string__SubString)
* [std::string::Trim (trim)](#std__string__Trim)
* [std::string::TrimEnd (trim_end)](#std__string__TrimEnd)
* [std::string::TrimStart (trim_start)](#std__string__TrimStart)
* [std::test::Assert (assert)](#std__test__Assert)
* [std::test::AssertEquals (assert_eq)](#std__test__AssertEquals)
* [std::test::AssertError (assert_error)](#std__test__AssertError)
* [std::test::AssertFail (assert_fail)](#std__test__AssertFail)
* [std::test::AssertFalse (assert_false)](#std__test__AssertFalse)
* [std::test::TestDirectory (test_directory)](#std__test__TestDirectory)
* [std::thread::Sleep (sleep)](#std__thread__Sleep)
* [std::time::CurrentTimeMillies (current_time)](#std__time__CurrentTimeMillies)


<a name="std__Echo"></a>
## std::Echo
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

<a name="std__Eval"></a>
## std::Eval
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

<a name="std__ForIn"></a>
## std::ForIn
```sh
args = array a b c
for arg in ${args}
    # commands
end
release args
```

The for/in command enables to iterate over an array (see [array command](#std__collections__Array)).<br>
The first argument will contain the current iteration value from the array.<br>
Once all values have been read, it will exit the loop.

#### Parameters

* for
  * The variable name which will hold the current iteration value
  * The string "in"
  * The handle to the array of values to iterate
* end - no parameters

#### Return Value

None

#### Examples

```sh
# Simple example iteration over the list of letters:
args = array a b c

for arg in ${args}
    echo current arg is: ${arg}
end

release args

# Example nested loops:
args = array 1 2 3
for i in ${args}
    for j in ${args}
        echo i: ${i} j: ${j}
    end
end
```


#### Aliases:
for

<a name="std__Function"></a>
## std::Function
```sh
function my_function
    # function content
    return output
end
```

This command provides the function language feature as a set of commands:

* function - Defines a function start block
* end - Defines the end of the function block
* return - Allows to exist a function at any point and return an output
* *function name* - Dynamically created commands based on the function name which are used to invoke the function code.

When a function command is detected, it will search for the end command that comes after.<br>
That entire block is considered the function code block (functions cannot be nested in outer functions)<br>

In order to invoke the function, simply call the function name with any amount of paramters.<br>
Those parameters will be set as $1, $2, ... and so on.<br>
Since variables are global, it will overwrite any older values stored in those variables.<br>

To exist a function and return a value, simply use the **return** command with the value you want to return.<br>
The variable that was used when the function was originally called, will now store that value.<br>
The return command can be used to exist early without any value.<br>
In case the code reached the **end** call, the function will exist but will return not value.

#### Parameters

* function - The function name used later on to invoke the function
* end - no parameters
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
end

# function invocation
hello_world

# Example of calling a function and returning a value
function get_hello_world
    return "hello world"
end

# function invocation
text = get_hello_world

# this will print "hello world"
echo ${text}

# Example of passing arguments
function print_input
    # $1 is set with the value 'hello'
    # $2 is set with the value 'world'
    echo ${1} ${2}
end

print_input hello world

# Functions can call other functions
function get_one
    return 1
end

function get_number
    number = get_one
    return ${number}
end

output = get_number

# this will print 1
echo ${output}
```


#### Aliases:
function, fn

<a name="std__GoTo"></a>
## std::GoTo
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

<a name="std__If"></a>
## std::If
```sh
if command|value
    # commands
elseif command|value
    # commands
else
    # commands
end
```

This command provides the if/elseif/else condition language feature as a set of commands:

* if - Defines an if condition
* elseif - Defines optional secondary condition blocks
* else - Optinoal fallback block
* end - Defines the end of the entire if/else block

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
* else/end - no parameters

#### Return Value

None

#### Examples

```sh
# Simple example of an if statement that evaluates the argument value as true and echos "in if"
if true
    echo in if
end

# Example of using **not** command to reverse the output value
if not false
    echo in if
end

# Example of an if statement that evaluates the command as true and echos "in if"
if set true
    echo in if
end

# Example of if condition returning a falsy result and navigation goes to the else block which echos "in else"
if set false
    echo should not be here
else
    echo in else
end

# Example of if condition returning a falsy result and navigation goes to the elseif block has a truthy condition
if set false
    echo should not be here
elseif set true
    echo in else if
else
    echo should not be here
end

# Nested if example:
if set false
    echo should not be here
elseif set true
    echo in else if but not done yet

    if set true
        echo nested if
    end
else
    echo should not be here
end
```


#### Aliases:
if

<a name="std__IsDefined"></a>
## std::IsDefined
```sh
var = is_defined key
```

Returns true if the provided variable name (not value) exists.

#### Parameters

The variable name.

#### Return Value

True if the variable is defined.

#### Examples

```sh
key = set "hello world"
exists = is_defined key
```


#### Aliases:
is_defined

<a name="std__Not"></a>
## std::Not
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

<a name="std__ReadUserInput"></a>
## std::ReadUserInput
```sh
var = read
```

Reads the user input into the output variable.<br>
If the user didn't insert any input, none will be returned.

#### Parameters

None

#### Return Value

The user input or none if no input was entered.

#### Examples

```sh
echo Enter Full Name:
name = read

if is_empty ${name}
    echo You didn't enter any value
else
    echo Your name is: ${name}
end
```


#### Aliases:
read

<a name="std__Release"></a>
## std::Release
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

<a name="std__Set"></a>
## std::Set
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

<a name="std__ShowCommandDocumentation"></a>
## std::ShowCommandDocumentation
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

<a name="std__alias__Set"></a>
## std::alias::Set
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

<a name="std__alias__Unset"></a>
## std::alias::Unset
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

<a name="std__collections__Array"></a>
## std::collections::Array
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

<a name="std__collections__ArrayConcat"></a>
## std::collections::ArrayConcat

```sh
handle = array_concat [handle]*
```

Concats all provided arrays and returns a handle to a new array with all items.

#### Parameters

Any number of array handles.

#### Return Value

A handle to the new array.

#### Examples

```sh
input1 = range 1 4
input2 = range 4 6
input3 = range 6 8

# new array will contain values from 1-7
arr = array_concat ${input1} ${input2} ${input3}
```


#### Source:

```sh
for scope::array_concat::arg in ${scope::array_concat::arguments}
    if not is_array ${scope::array_concat::arg}
        trigger_error "Invalid input, non array handle or array not found."
    end
end

scope::array_concat::array = array

for scope::array_concat::arg in ${scope::array_concat::arguments}
    for scope::array_concat::item in ${scope::array_concat::arg}
        array_push ${scope::array_concat::array} ${scope::array_concat::item}
    end
end

set ${scope::array_concat::array}
```


#### Aliases:
array_concat

<a name="std__collections__ArrayIsEmpty"></a>
## std::collections::ArrayIsEmpty

```sh
var = array_is_empty handle
```

Returns true if the provided array handle is an empty array.

#### Parameters

The array handle.

#### Return Value

True if the provided handle belongs to an empty array.

#### Examples

```sh
values = array
out = array_is_empty ${values}
```


#### Source:

```sh
scope::array_is_empty::length = array_length ${scope::array_is_empty::argument::1}
equals 0 ${scope::array_is_empty::length}
```


#### Aliases:
array_is_empty

<a name="std__collections__ArrayJoin"></a>
## std::collections::ArrayJoin

```sh
var = array_join handle separator
```

Joins all values in the provided array with the provided separator in between each value.

#### Parameters

* An array handle
* The separator to put between each item pair

#### Return Value

The joined string value

#### Examples

```sh
function test_to_string
    arr = array hello world
    string = array_join ${arr} ", "

    release ${arr}

    assert_eq ${string} "hello, world"
end

function test_numbers
    arr = range 1 5
    string = array_join ${arr} ", "

    release ${arr}

    assert_eq ${string} "1, 2, 3, 4"
end

function test_empty_separator
    arr = range 1 5
    string = array_join ${arr} ""

    release ${arr}

    assert_eq ${string} "1234"
end
```


#### Source:

```sh
if not is_array ${scope::array_join::argument::1}
    trigger_error "Invalid input, non array handle or array not found."
end

if not array_is_empty ${scope::array_join::argument::1}
    for scope::array_join::item in ${scope::array_join::argument::1}
        scope::array_join::string = set "${scope::array_join::string}${scope::array_join::item}${scope::array_join::argument::2}"
    end

    if not is_empty ${scope::array_join::argument::2}
        scope::array_join::separatorlen = strlen ${scope::array_join::argument::2}
        scope::array_join::stringlen = strlen ${scope::array_join::string}
        scope::array_join::offset = calc ${scope::array_join::stringlen} - ${scope::array_join::separatorlen}
        scope::array_join::string = substring ${scope::array_join::string} 0 ${scope::array_join::offset}
    end
end

set ${scope::array_join::string}
```


#### Aliases:
array_join

<a name="std__collections__ArrayLength"></a>
## std::collections::ArrayLength
```sh
var = array_length handle
```

Returns the array length based on the provided array handle.

#### Parameters

The array handle.

#### Return Value

The array length.

#### Examples

```sh
handle = array a b c "d e"
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}

handle = range 0 10
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}
```


#### Aliases:
array_length, arrlen

<a name="std__collections__ArrayPop"></a>
## std::collections::ArrayPop
```sh
var = array_pop handle
```

Returns the last element of the array or none if the array is empty.

#### Parameters

The array handle.

#### Return Value

The last element of the array or none if the array is empty.

#### Examples

```sh
handle = array 1 2 3
last_element = array_pop ${handle}
assert_eq ${last_element} 3
```


#### Aliases:
array_pop

<a name="std__collections__ArrayPush"></a>
## std::collections::ArrayPush
```sh
var = array_push handle value
```

Pushes an additional value to an existing array.

#### Parameters

The array handle.

#### Return Value

True if a new value was pushed.

#### Examples

```sh
handle = array 1 2 3
array_push ${handle} 4
last_element = array_pop ${handle}
assert_eq ${last_element} 4
```


#### Aliases:
array_push

<a name="std__collections__IsArray"></a>
## std::collections::IsArray
```sh
var = array_length handle
```

Returns the array length based on the provided array handle.

#### Parameters

The array handle.

#### Return Value

The array length.

#### Examples

```sh
handle = array a b c "d e"
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}

handle = range 0 10
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}
```


#### Aliases:
is_array

<a name="std__collections__Range"></a>
## std::collections::Range
```sh
handle = range start end
```

Creates an array from the input start and end range values and returns a handle to that array.<br>
This handle can be passed to other commands which support arrays using handles.<br>
Once the array is no longer used, it should be released using the **release** command.

#### Parameters

* The start numeric value
* The end numeric value which cannot be smaller than the start value.

#### Return Value

A handle to the array.

#### Examples

```sh
handle = range 1 10

# once done we should release the handle
release ${handle}
```


#### Aliases:
range

<a name="std__collections__ReadProperties"></a>
## std::collections::ReadProperties
```sh
count = read_properties [--prefix key] text
```

Parses the properties (based on java properties format) text and sets them as variables.<br>
This command will also return the count of properties read.<br>
If prefix is provided, all properties read, will be stored as variables with the **prefix.** as their prefix.

#### Parameters

* Optional --prefix and the prefix value
* The text to parse.

#### Return Value

The properties count.

#### Examples

```sh
count = read_properties "a=1\nb=2\na.b.c=3"
assert_eq ${count} 3

assert_eq ${a} 1
assert_eq ${b} 2
assert_eq ${a.b.c} 3

count = read_properties --prefix config a=1\nb=2\na.b.c=3
assert_eq ${count} 3

assert_eq ${config.a} 1
assert_eq ${config.b} 2
assert_eq ${config.a.b.c} 3
```


#### Aliases:
read_properties

<a name="std__collections__WriteProperties"></a>
## std::collections::WriteProperties
```sh
text = write_properties [names]
```

Creates a properties string from the provided list of variable names (not values).

#### Parameters

A list of variable names.

#### Return Value

The properties text value.

#### Examples

```sh
a = set 1
b = set 2
a.b.c = set 3

# text will be equal to:
# a=1
# b=2
# a.b.c=3
text = write_properties a b a.b.c
```


#### Aliases:
write_properties

<a name="std__debug__DuckscriptSDKVersion"></a>
## std::debug::DuckscriptSDKVersion
```sh
var = duckscript_sdk_version
```

Returns the duckscript SDK version.

#### Parameters

None

#### Return Value

The duckscript SDK version.

#### Examples

```sh
version = duckscript_sdk_version 
```


#### Aliases:
duckscript_sdk_version

<a name="std__debug__DuckscriptVersion"></a>
## std::debug::DuckscriptVersion
```sh
var = duckscript_version
```

Returns the duckscript runtime version.

#### Parameters

None

#### Return Value

The duckscript runtime version.

#### Examples

```sh
version = duckscript_version 
```


#### Aliases:
duckscript_version

<a name="std__debug__DumpInstructions"></a>
## std::debug::DumpInstructions
```sh
value = dump_instructions
```

Returns all script instructions structure (not script text) in textual form.

#### Parameters

None

#### Return Value

The script instructions.

#### Examples

```sh
value = dump_instructions
found = contains ${value} dump_instructions
assert found
```


#### Aliases:
dump_instructions

<a name="std__debug__DumpState"></a>
## std::debug::DumpState
```sh
value = dump_state
```

Returns all script state in textual form.

#### Parameters

None

#### Return Value

The script state.

#### Examples

```sh
numbers = range -5 15

text = dump_instructions
found = contains ${text} -5
assert found
```


#### Aliases:
dump_state

<a name="std__debug__DumpVariables"></a>
## std::debug::DumpVariables
```sh
value = dump_variables
```

Returns all script variables in textual form.

#### Parameters

None

#### Return Value

The script variables.

#### Examples

```sh
one = set 1
two = set 2
values = array 1 2 yes true
numbers = range -5 15

text = dump_variables
found = contains ${text} two
assert found
found = contains ${text} 2
assert found
found = contains ${text} handle
assert found
```


#### Aliases:
dump_variables

<a name="std__env__GetHomeDirectory"></a>
## std::env::GetHomeDirectory
```sh
var = get_home_dir
```

Returns the user home directory path.<br>
In case of any error, false will be returned.

#### Parameters

None

#### Return Value

The user home directory path or false in case of any error.

#### Examples

```sh
directory = get_home_dir
```


#### Aliases:
get_home_dir

<a name="std__env__GetVar"></a>
## std::env::GetVar
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

<a name="std__env__PrintCurrentDirectory"></a>
## std::env::PrintCurrentDirectory
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
pwd, print_current_directory

<a name="std__env__SetCurrentDirectory"></a>
## std::env::SetCurrentDirectory
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
cd, set_current_dir, set_current_directory

<a name="std__env__SetVar"></a>
## std::env::SetVar
```sh
var = set_env key value
```

Sets the environment variable defined by the provided key to the provided value.

#### Parameters

Two arguments are required:

* key - The name of the environment variable to set
* value - The new environment variable value

#### Return Value

true if successful

#### Examples

```sh
set_env HOME /usr/me
```


#### Aliases:
set_env

<a name="std__env__UnsetVar"></a>
## std::env::UnsetVar
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

<a name="std__error__GetLastError"></a>
## std::error::GetLastError
```sh
var = get_last_error
```

In case of any runtime error, this function will return the error message.

#### Parameters

None

#### Return Value

The last error message or none

#### Examples

```sh
# This will trigger an error
assert_fail

error = get_last_error
echo Error Message: ${error}
```


#### Aliases:
get_last_error

<a name="std__error__GetLastErrorLine"></a>
## std::error::GetLastErrorLine
```sh
var = get_last_error_line
```

In case of any runtime error, this function will return the error line (if available).

#### Parameters

None

#### Return Value

The last error line or none

#### Examples

```sh
# This will trigger an error
assert_fail

line = get_last_error_line
echo Error Line: ${line}
```


#### Aliases:
get_last_error_line

<a name="std__error__GetLastErrorSource"></a>
## std::error::GetLastErrorSource
```sh
var = get_last_error_source
```

In case of any runtime error, this function will return the error source (such as file name) if available.

#### Parameters

None

#### Return Value

The last error source or none

#### Examples

```sh
# This will trigger an error
assert_fail

source = get_last_error_source
echo Error Source File: ${source}
```


#### Aliases:
get_last_error_source

<a name="std__error__SetError"></a>
## std::error::SetError
```sh
set_error message
```

Sets the last error which is accessible via get_last_error.<br>
This command will not trigger the on_error command flow.

#### Parameters

The error message.

#### Return Value

None

#### Examples

```sh
set_error "my error message"

error = get_last_error

assert_eq ${error} "my error message"
```


#### Aliases:
set_error

<a name="std__error__SetExitOnError"></a>
## std::error::SetExitOnError
```sh
var = exit_on_error value
```

Enables to cause the script execution to stop in case of any error.<br>
By default all errors simply trigger the on_error command which the default SDK stores and provides access to.<br>
However, with this command you can change the on_error command to instead stop the script execution.

#### Parameters

If no argument is provided, it will return the current state.<br>
If an argument is provided, it will modify the state and return it as true/false.

#### Return Value

The current/updated state as true/false value

#### Examples

```sh
# Get current state
will_exit = exit_on_error
echo Current state: ${will_exit}

# Update the current state
will_exit = exit_on_error true
echo Current state: ${will_exit}
```


#### Aliases:
exit_on_error, set_exit_on_error

<a name="std__error__TriggerError"></a>
## std::error::TriggerError
```sh
trigger_error [message]
```

Triggers an error that will trigger the on_error flow.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
trigger_error "my error message"
error = get_last_error
assert_eq ${error} "my error message"
```


#### Aliases:
trigger_error

<a name="std__fs__Append"></a>
## std::fs::Append
```sh
result = appendfile file text
```

This command enables to write the provided text into the requested file.<br>
It will return true/false value based if it was able to write the text to the file.<br>
In case the file doesn't exist, it will create it.<br>
If the file exists, it will append the text to it.

#### Parameters

* The target file
* The text content to write

#### Return Value

true/false based if it was able to write the text to the file.

#### Examples

```sh
out = appendfile ./target/tests/writefile.txt "line 1\nline 2"
```


#### Aliases:
appendfile

<a name="std__fs__CopyPath"></a>
## std::fs::CopyPath
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

<a name="std__fs__CreateDirectory"></a>
## std::fs::CreateDirectory
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

<a name="std__fs__CreateEmptyFile"></a>
## std::fs::CreateEmptyFile
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

<a name="std__fs__DeleteEmptyDirectory"></a>
## std::fs::DeleteEmptyDirectory
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

<a name="std__fs__DeletePath"></a>
## std::fs::DeletePath
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

<a name="std__fs__GetCanonicalPath"></a>
## std::fs::GetCanonicalPath
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

<a name="std__fs__GetFileName"></a>
## std::fs::GetFileName
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

<a name="std__fs__GetParentDirectory"></a>
## std::fs::GetParentDirectory
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

<a name="std__fs__List"></a>
## std::fs::List
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

<a name="std__fs__MovePath"></a>
## std::fs::MovePath
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

<a name="std__fs__Print"></a>
## std::fs::Print
```sh
var = cat [file]+
```

The cat command will print out the requested file/s.<br>
In addition it will also return the value to the output variable.

#### Parameters

Multiple file paths.

#### Return Value

The file content or none if the file does not exist.

#### Examples

```sh
cat ./docs/sdk.md
```


#### Aliases:
cat

<a name="std__fs__Read"></a>
## std::fs::Read
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

<a name="std__fs__Write"></a>
## std::fs::Write
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

<a name="std__math__Calc"></a>
## std::math::Calc
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

<a name="std__math__GreaterThan"></a>
## std::math::GreaterThan
```sh
var = greater_than left right
```

This command returns true/false based on left > right calculation.

#### Parameters

Two numeric values to compare.

#### Return Value

True if first argument is bigger than second argument.

#### Examples

```sh
result = greater_than 2 1.5
```


#### Aliases:
greater_than

<a name="std__math__LessThan"></a>
## std::math::LessThan
```sh
var = less_than left right
```

This command returns true/false based on left < right calculation.

#### Parameters

Two numeric values to compare.

#### Return Value

True if first argument is smaller than second argument.

#### Examples

```sh
result = less_than 1 1.5
```


#### Aliases:
less_than

<a name="std__net__Hostname"></a>
## std::net::Hostname
```sh
var = hostname
```

Returns the hostname.

#### Parameters

None

#### Return Value

The hostname

#### Examples

```sh
name = hostname
```


#### Aliases:
hostname

<a name="std__net__HttpClient"></a>
## std::net::HttpClient
```sh
var = http_client [--method method] [--payload payload] [--output-file file] URL
```

Invokes a HTTP request.<br>
The request method by default is GET but can be modified by the ```--method``` parameter.<br>
The ```--output-file``` parameter will redirect a valid response output to the provided file, otherwise all response text will be set to the
output variable.<br>
When redirecting to file, the output would be the response size.<br>
The ```--payload``` parameter enables to pass a payload to POST http requests.<br>
In case of errors or error HTTP response codes, false will be returned.

#### Parameters

* Optional HTTP Method, for example ```--method GET``` or ```--method POST``` (currently only GET and POST are supported).
* Optional post payload via ```--payload``` parameter.
* Optional redirection of output to file via ```--output-file``` parameter.
* The target URL

#### Return Value

The response text or in case of output redirection to file, the response size.<br>
In case of errors, it will return false.

#### Examples

```sh
function test_get
    response = http_client https://www.rust-lang.org/

    found = contains ${response} Rust

    assert ${found}
end

function test_get_to_file
    file = set ./target/_duckscript_test/http_client/page.html
    rm ${file}

    response_size = http_client --output-file ${file} https://www.rust-lang.org/

    response = readfile ${file}
    found = contains ${response} Rust

    assert ${found}
    assert ${response_size}
end

function test_post
    payload = set {\"login\":\"login\",\"password\":\"password\"}
    response = http_client --method POST --payload ${payload} https://reqbin.com/echo/post/json

    found = contains ${response} success

    assert ${found}
end
```


#### Aliases:
http_client

<a name="std__net__WGet"></a>
## std::net::WGet

```sh
var = wget [--method=HTTP-method] [--post-data=payload] [-O file] URL
```

Invokes a HTTP request.<br>
The request method by default is GET but can be modified by the ```--method``` parameter.<br>
The ```-O``` parameter will redirect a valid response output to the provided file, otherwise all response text will be set to the
output variable.<br>
When redirecting to file, the output would be the response size.<br>
The ```--post-data``` parameter enables to pass a payload to POST http requests.<br>
In case of errors or error HTTP response codes, false will be returned.

#### Parameters

* Optional HTTP Method, for example --method=HTTP-GET or --method=HTTP-POST (currently only GET and POST are supported).
* Optional post payload via ```--post-data``` parameter.
* Optional redirection of output to file via ```-O``` parameter.
* The target URL

#### Return Value

The response text or in case of output redirection to file, the response size.<br>
In case of errors, it will return false.

#### Examples

```sh
function test_get
    response = wget https://www.rust-lang.org/

    found = contains ${response} Rust

    assert ${found}
end

function test_get_to_file
    file = set ./target/_duckscript_test/wget/page.html
    rm ${file}

    response_size = wget -O ${file} https://www.rust-lang.org/

    response = readfile ${file}
    found = contains ${response} Rust

    assert ${found}
    assert ${response_size}
end

function test_post
    payload = set {\"login\":\"login\",\"password\":\"password\"}
    response = wget --method=HTTP-POST --post-data=${payload} https://reqbin.com/echo/post/json

    found = contains ${response} success

    assert ${found}
end
```


#### Source:

```sh
scope::wget::url = array_pop ${scope::wget::arguments}

scope::wget::method = set GET

scope::wget::lookingfor = set flag
for scope::wget::arg in ${scope::wget::arguments}
    if equals ${scope::wget::lookingfor} flag
        if starts_with ${scope::wget::arg} --method=HTTP-
            scope::wget::len = strlen --method=HTTP-
            scope::wget::method = substring ${scope::wget::arg} ${scope::wget::len}
        elif starts_with ${scope::wget::arg} --post-data=
            scope::wget::len = strlen --post-data=
            scope::wget::payload = substring ${scope::wget::arg} ${scope::wget::len}
        elif equals ${scope::wget::arg} -O
            scope::wget::lookingfor = set file
        end
    elif equals ${scope::wget::lookingfor} file
        scope::wget::file = set ${scope::wget::arg}
        scope::wget::lookingfor = set flag
    end
end

http_client --method "${scope::wget::method}" --output-file "${scope::wget::file}" --payload "${scope::wget::payload}" ${scope::wget::url}
```


#### Aliases:
wget

<a name="std__process__Execute"></a>
## std::process::Execute
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

<a name="std__process__Exit"></a>
## std::process::Exit
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
exit, quit, q

<a name="std__process__ProcessID"></a>
## std::process::ProcessID
```sh
var = pid
```

Returns the current process ID.

#### Parameters

None

#### Return Value

The current process ID.

#### Examples

```sh
id = pid
```


#### Aliases:
pid, process_id

<a name="std__process__Watchdog"></a>
## std::process::Watchdog
```sh
count = watchdog [--max-retries value] [--interval value] -- command [arguments]*
```

Executes the provided native command and arguments.<br>
In case the command existed it will be executed again up to the max retries provided.<br>
The watchdog will wait the specified time in milliseconds between invocations.<br>
In case of an invalid command, the watchdog will not reattempt the invocation and will exit without retries.

#### Parameters

* --max-retries - Positive value of max retries (excluding the first invocation). value <= 0 for unlimited retries. Default is unlimited.
* --interval - The amount in milliseconds between retries. 0 for no waiting between invocations. Default is no wait.
* The command to executed (preceded by a **--** separator).
* The command arguments.

#### Return Value

The amount of invocations or false in case of any error.

#### Examples

```sh
count = watchdog --max-retries 0 -- echo test
assert_eq ${count} 1

count = watchdog --max-retries 3 --interval 10 -- echo test
assert_eq ${count} 4
```


#### Aliases:
watchdog

<a name="std__scope__Clear"></a>
## std::scope::Clear
```sh
clear_scope name
```

Clears all variables which are prefixed with the provided name + ::.<br>
For example, if the value provided is **my_scope** all variables that start with **my_scope::** will be removed.

#### Parameters

The scope name.

#### Return Value

None.

#### Examples

```sh
testscope = set true
testscope::1 = set 1
testscope::subscope::1 = set 1

assert_eq ${testscope} true
defined = is_defined testscope::1
assert ${defined}
assert_eq ${testscope::1} 1
defined = is_defined testscope::subscope::1
assert ${defined}
assert_eq ${testscope::subscope::1} 1

clear_scope testscope

assert_eq ${testscope} true

defined = is_defined testscope::1
assert_false ${defined}
defined = is_defined testscope::subscope::1
assert_false ${defined}
```


#### Aliases:
clear_scope

<a name="std__string__Concat"></a>
## std::string::Concat

```sh
var = concat [value]*
```

Concats the provided input into a single string and returns it.

#### Parameters

Any number of values to concat.

#### Return Value

The result of the concatenation of all input values.

#### Examples

```sh
output = concat 1 2 3 4
assert_eq ${output} 1234

output = concat 1 "2 3" 4
assert_eq ${output} "12 34"
```


#### Source:

```sh
scope::concat::output = set ""
for scope::concat::arg in ${scope::concat::arguments}
    scope::concat::output = set "${scope::concat::output}${scope::concat::arg}"
end

set ${scope::concat::output}
```


#### Aliases:
concat

<a name="std__string__Contains"></a>
## std::string::Contains
```sh
var = contains all partial
```

Returns true if the first argument contains the value of the second argument.

#### Parameters

* The full text to search in
* The text to search for

#### Return Value

**true** if contains.

#### Examples

```sh
# valid conditions
result = contains abcd bc

value = set "some text"
result = contains ${value} "me tex"

# will return false
result = contains abcd b1c
```


#### Aliases:
contains

<a name="std__string__EndsWith"></a>
## std::string::EndsWith
```sh
var = ends_with all partial
```

Returns true if the first argument ends with the value of the second argument.

#### Parameters

* The full text to search in
* The suffix text to search for

#### Return Value

**true** if ends with.

#### Examples

```sh
# valid conditions
result = ends_with abcd abc

value = set "some text"
result = ends_with ${value} "me text"

# will return false
result = ends_with abcd abc
```


#### Aliases:
ends_with

<a name="std__string__Equals"></a>
## std::string::Equals
```sh
var = eq value1 value2
```

Returns true if both provided values are equal.

#### Parameters

Two values to evaluate if they are equal

#### Return Value

**true** if equal.

#### Examples

```sh
# valid conditions
is_same = eq yes yes
is_same = eq false false

value = set "some text"
is_same = eq ${value} "some text"

# will return false
is_same = eq 1 2
```


#### Aliases:
equals, eq

<a name="std__string__IndexOf"></a>
## std::string::IndexOf
```sh
var = indexof full_text text_to_find
```

This command will attempt to find the text from the second argument inside the text in the first argument.<br>
If found, an index value will be returned, otherwise none is returned.

#### Parameters

* The text to search in
* The text to find

#### Return Value

The index of the text found or none if not found.

#### Examples

```sh
index = indexof "    some  text   " some 
```


#### Aliases:
indexof

<a name="std__string__IsEmpty"></a>
## std::string::IsEmpty
```sh
var = is_empty value
```

Returns true if the provided value is none or an empty string.

#### Parameters

The value to validate.

#### Return Value

True if the provided value is none or an empty string.

#### Examples

```sh
value = set "hello world"
empty = is_empty ${value}
```


#### Aliases:
is_empty

<a name="std__string__LastIndexOf"></a>
## std::string::LastIndexOf
```sh
var = last_indexof full_text text_to_find
```

This command will attempt to find the text from the second argument inside the text in the first argument.<br>
If found, an index value will be returned, otherwise none is returned.<br>
Unlike the **indexof** command, this command will search for text starting at the end, going backwards.

#### Parameters

* The text to search in
* The text to find

#### Return Value

The index of the text found or none if not found.

#### Examples

```sh
index = last_indexof "    some  text   " some
```


#### Aliases:
last_indexof

<a name="std__string__Length"></a>
## std::string::Length
```sh
var = length text
```

Returns the text length.

#### Parameters

The text to extract the length from.

#### Return Value

The text length value.

#### Examples

```sh
len = length "Hello World"
```


#### Aliases:
length, strlen

<a name="std__string__Replace"></a>
## std::string::Replace
```sh
var = replace text from to
```

Returns new value of text after replacing all from values to the provided to values.

#### Parameters

* The full text
* The from text
* The to text

#### Return Value

The updated text.

#### Examples

```sh
text = set "my large text value with lots of text"
updated = replace ${text} text stuff

assert_eq ${updated} "my large stuff value with lots of stuff"
```


#### Aliases:
replace

<a name="std__string__Split"></a>
## std::string::Split
```sh
handle = split text pattern
```

Splits the provided text based on the provided pattern and return a handle the
created array with all the splitted values.

#### Parameters

* The text to split
* The pattern to split by

#### Return Value

A handle to the values array.

#### Examples

```sh
handle = split a23b23c23d23e 23

len = array_length ${handle}

value = array_pop ${handle}
assert_eq ${value} e
value = array_pop ${handle}
assert_eq ${value} d
value = array_pop ${handle}
assert_eq ${value} c
value = array_pop ${handle}
assert_eq ${value} b
value = array_pop ${handle}
assert_eq ${value} a

release ${handle}

assert_eq ${len} 5
```


#### Aliases:
split

<a name="std__string__StartsWith"></a>
## std::string::StartsWith
```sh
var = starts_with all partial
```

Returns true if the first argument starts with the value of the second argument.

#### Parameters

* The full text to search in
* The prefix text to search for

#### Return Value

**true** if starts with.

#### Examples

```sh
# valid conditions
result = starts_with abcd abc

value = set "some text"
result = starts_with ${value} "some te"

# will return false
result = starts_with abcd bcd
```


#### Aliases:
starts_with

<a name="std__string__SubString"></a>
## std::string::SubString
```sh
var = substring text
var = substring text start end
var = substring text start
var = substring text -end
```

The substring command will create a new string value from the text provided in the range requested.

#### Parameters

* The text to substring from
* Additional parameters
    * None - start index is 0 and end index is the text length
    * Two arguments - First is the start index and second is the end index
    * One argument
        * If >= 0 it defines the start index and end index is the text length
        * If < 0 it defines the end index going backwards from the end of the text. Start index is 0.

#### Return Value

The substring value or false in case of error.

#### Examples

```sh
# string is 'Hello World'
string = substring "Hello World"
echo ${string}

# string is 'll'
string = substring "Hello World" 2 4
echo ${string}

# string is 'llo World'
string = substring "Hello World" 2
echo ${string}

# string is 'Hello W'
string = substring "Hello World" -4
echo ${string}
```


#### Aliases:
substring

<a name="std__string__Trim"></a>
## std::string::Trim
```sh
var = trim value
```

Returns the provided value with leading and trailing whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "some  text"
trimmed = trim "  some  text   "
```


#### Aliases:
trim

<a name="std__string__TrimEnd"></a>
## std::string::TrimEnd
```sh
var = trim_end value
```

Returns the provided value with trailing whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "  some  text"
trimmed = trim_end "  some  text   "
```


#### Aliases:
trim_end

<a name="std__string__TrimStart"></a>
## std::string::TrimStart
```sh
var = trim_start value
```

Returns the provided value with leading whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "some  text   "
trimmed = trim_start "  some  text   "
```


#### Aliases:
trim_start

<a name="std__test__Assert"></a>
## std::test::Assert
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

# error conditions (each one will break the execution)
assert
assert false
assert 0
assert false "This is my error message"
```


#### Aliases:
assert

<a name="std__test__AssertEquals"></a>
## std::test::AssertEquals
```sh
assert_eq value1 value2 [error message]
```

Used to validate the input is the same.<br>
If they are not, the command will exist with an error.

#### Parameters

* Two values to evaluate if they are equal
* Optional error message

#### Return Value

**true** if equal.

#### Examples

```sh
# valid conditions
assert_eq yes yes
assert_eq false false

value = set "some text"
assert_eq ${value} "some text"

# error conditions (each one will break the execution)
assert_eq 1 2
assert_eq 1 2 "This is my error message"
```


#### Aliases:
assert_eq

<a name="std__test__AssertError"></a>
## std::test::AssertError
```sh
assert_error [error message]
```

This command will cause a runtime error which will not stop the script execution.<br>
If error message is provided, it will be used as part of the error output.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
assert_error

assert_error "This is my error message"
```


#### Aliases:
assert_error

<a name="std__test__AssertFail"></a>
## std::test::AssertFail
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

<a name="std__test__AssertFalse"></a>
## std::test::AssertFalse
```sh
assert_false value [error message]
```

Used to validate the input is falsy.<br>
If the value is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.

#### Parameters

* The value to evaluate
* Optional error message

#### Return Value

**true** if falsy.

#### Examples

```sh
# valid conditions
assert_false
assert_false false
assert_false 0
assert_false false "This is my error message"

# error conditions (each one will break the execution)
assert_false ok
assert_false true
assert_false yes

value = set "some text"
assert_false ${value}
```


#### Aliases:
assert_false

<a name="std__test__TestDirectory"></a>
## std::test::TestDirectory
```sh
test_directory directory [pattern]
```

This command can be used to run unit tests written in duckscript.<br>
It will run all duckscript files in the directory tree ending with **test.ds** and for each file, it will run
all functions that start with **test_**.<br>
Each such function is considered as a test and can run any type of code and check itself using assert commands.

#### Parameters

* The root directory of all test files (all files ending with **test.ds** in the directory tree will be checked)
* Optional pattern for the file name or test function to limit invocation of only those tests.

#### Return Value

**true** if successful.

#### Examples

This is an example of a test function:

```sh
function test_set_get_unset
    unset_env TEST_SET_GET_UNSET
    value = get_env TEST_SET_GET_UNSET
    assert_false ${value}

    value = set_env TEST_SET_GET_UNSET "test value"
    assert ${value}
    value = get_env TEST_SET_GET_UNSET
    assert_eq ${value} "test value"
end
```


#### Aliases:
test_directory

<a name="std__thread__Sleep"></a>
## std::thread::Sleep
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

<a name="std__time__CurrentTimeMillies"></a>
## std::time::CurrentTimeMillies
```sh
var = current_time
```

Returns the current time in milliseconds (from January 1, 1970 UTC).

#### Parameters

None

#### Return Value

The current time in milliseconds.

#### Examples

```sh
result = current_time
echo ${result}
```


#### Aliases:
current_time

### License
Developed by Sagie Gur-Ari and licensed under the
[Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
