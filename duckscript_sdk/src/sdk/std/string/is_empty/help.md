```sh
var = is_empty value
```

Returns true if the provided value is none or an empty string.

### Parameters

The value to validate.

### Return Value

True if the provided value is none or an empty string.

### Examples

```sh
value = set "hello world"
empty = is_empty ${value}
```
