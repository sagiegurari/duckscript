```sh
var = snakecase text
```

Converts the provided string into snake case.
All non-alphanumeric characters are ignored.

### Parameters

The string to convert.

### Return Value

The converted string.

### Examples

```sh
string = snakecase "Hello, World!"
assert_eq ${string} "hello_world"
```

