```sh
var = read
```

Reads the user input into the output variable.<br>
If the user didn't insert any input, none will be returned.

### Parameters

None

### Return Value

The user input or none if no input was entered.

### Examples

```sh
echo Enter Full Name:
name = read

if is_empty ${name}
    echo You didn't enter any value
else
    echo Your name is: ${name}
end
```
