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
