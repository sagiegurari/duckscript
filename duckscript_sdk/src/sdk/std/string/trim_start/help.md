```sh
var = trim_start value
```

Returns the provided value with leading whitespace removed.

### Parameters

The value to trim.

### Return Value

The trimmed value. If no input provided, this command will return none.

### Examples

```sh
# trimmed will now hold "some  text   "
trimmed = trim_start "  some  text   "
```
