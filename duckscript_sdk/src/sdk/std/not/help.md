```sh
output = not [command|value|condition]
```

Enables to switch falsy to true and truthy to false.<br>
The **not** commands accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command
* A condition statement

If the result is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It will return true, otherwise it will return false.

A condition statement is made up of values, or/and keywords and '('/')' groups.<br>
Each must be separated with a space character.

### Parameters

A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.

### Return Value

The switched value of the input.

### Examples

```sh
fn test_not_true
    value = not true

    assert_false ${value}
end

fn test_not_false
    value = not false

    assert ${value}
end

fn test_not_command_true
    value = not set true

    assert_false ${value}
end

fn test_not_command_false
    value = not set false

    assert ${value}
end

fn test_not_condition_true
    value = not true and false or true and false or ( true and true or false )

    assert_false ${value}
end

fn test_not_condition_false
    value = not true and false or true and false or ( true and true or false ) and false

    assert ${value}
end
```
