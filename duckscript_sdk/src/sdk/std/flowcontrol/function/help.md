```sh
fn my_function
    # function content
    return output
end

fn <scope> another_function
    # function content
end
```

This command provides the function language feature as a set of commands:

* function/fn - Defines a function start block
* end - Defines the end of the function block
* return - Allows to exit a function at any point and return an output
* *&lt;scope&gt;* - Optional annotation which enables to use a new scope during the function invocation.
* *function name* - Dynamically created commands based on the function name which are used to invoke the function code.

When a function command is detected, it will search for the end command that comes after.<br>
That entire block is considered the function code block (functions cannot be nested in outer functions)<br>

In order to invoke the function, simply call the function name with any amount of parameters.<br>
Those parameters will be set as ${1}, ${2}, ... and so on.<br>
Since variables are global, it will overwrite any older values stored in those variables.<br>

To exit a function and return a value, simply use the **return** command with the value you want to return.<br>
The variable that was used when the function was originally called, will now store that value.<br>
The return command can be used to exit early without any value.<br>
In case the code reached the **end** call, the function will exit but will not return a value.<br>

The *&lt;scope&gt;* annotation enables to start a new scope when running the function.<br>
All variables defined will not be available except the variables provided to the function as arguments.<br>
All variables created during the function invocation will be deleted once the function ends, except the return value.<br>
This enables a clean function invocation without impacting the global variables.

#### Parameters

* function - The function name used later on to invoke the function
* end - no parameters
* return - optional single parameter to return as an output of the function call
* *&lt;scope&gt;* - Optional annotation which enables to use a new scope during the function invocation.
* *function name* - Any number of arguments which will automatically be set as global variables: ${1}, ${2}, ... as so on.

#### Return Value

The function invocation returns the output provided by the return command.

#### Examples

```sh
# Simple example of a function definition which echo 'hello world' and exits.

# function start
fn hello_world
    echo hello world
end

# function invocation
hello_world

# Example of calling a function and returning a value
fn get_hello_world
    return "hello world"
end

# function invocation
text = get_hello_world

# this will print "hello world"
echo ${text}

# Example of passing arguments
# Also the function is with scope annotation so it has no access
# to any variable except those provided during the function invocation.
fn <scope> print_input
    # ${1} is set with the value 'hello'
    # ${2} is set with the value 'world'
    echo ${1} ${2}
end

print_input hello world

# Functions can call other functions
fn get_one
    return 1
end

fn get_number
    number = get_one
    return ${number}
end

output = get_number

# this will print 1
echo ${output}
```
