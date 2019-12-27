# Table of Contents
* [sdk::Echo](#sdk__Echo)
* [sdk::Function](#sdk__Function)
* [sdk::GoTo](#sdk__GoTo)
* [sdk::Set](#sdk__Set)
* [sdk::env::Get](#sdk__env__Get)
* [sdk::env::Set](#sdk__env__Set)
* [sdk::fs::Print](#sdk__fs__Print)
* [sdk::fs::Read](#sdk__fs__Read)


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
echo ${$1} ${$2}

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

### License
Developed by Sagie Gur-Ari and licensed under the
[Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
