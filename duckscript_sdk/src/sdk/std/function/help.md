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
