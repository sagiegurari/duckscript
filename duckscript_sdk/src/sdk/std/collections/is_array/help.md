```sh
var = is_array handle
```

Returns true if the provided value is an array handle.

### Parameters

The array handle.

### Return Value

True if the provided value is an array handle.

### Examples

```sh
arr = array 1 2 3

value = is_array ${arr}
assert ${value}

released = release ${arr}
assert ${released}
```
