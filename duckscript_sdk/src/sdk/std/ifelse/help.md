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
