```sh
while [command|value|condition]
    # commands
end
```

This command provides the while loop language feature as a set of commands:

* while - Defines a while condition and start of loop
* end - Defines the end of the while block

The while command accept either:

* A command with optional arguments and invokes it
* A single value which doesn't match any known command
* A condition statement

If the result is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.<br>
In case of falsy value, it will skip to the next line after the while block.<br>
If a truthy (non falsy) output is found, it will invoke the commands of that code block and go back to the start of the while condition.<br>

while blocks can be nested in other while blocks (see examples).

A condition statement is made up of values, or/and keywords and '('/')' groups.<br>
Each must be separated with a space character.

### Parameters

* while - A command and its arguments to invoke and evaluate its output, if a single value is provided an no such command exists, it is evaluated as a value.
* end - no parameters

### Return Value

None

### Examples

```sh
top_count = set 0
inner_count = set 0
counter = set 0
while not equals ${top_count} 10
    top_count = calc ${top_count} + 1
    inner_count = set 0

    while not equals ${inner_count} 10
        inner_count = calc ${inner_count} + 1
        counter = calc ${counter} + 1
    end
end

assert_eq ${counter} 100
```
