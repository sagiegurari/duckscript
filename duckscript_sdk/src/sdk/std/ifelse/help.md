```sh
if command
    # commands
elseif command
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

if and elseif commands accept a command with arguments and invokes it.<br>
If the result of the command is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)

It is considered falsy.<br>
In case of falsy value, it will skip to the next elseif/else block.<br>
If a truthy (non falsy) output is found, it will invoke the commands of that code block and ignore all other elseif/else blocks.<br>

if blocks can be nested in other if blocks (see examples).

#### Parameters

* if/elseif - A command and its arguments to invoke and evaluate its output
* else/end_if - no parameters

#### Return Value

None

#### Examples

Simple example of an if statement that evaluates to true and echos "in if"

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
