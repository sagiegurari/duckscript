```sh
var = is_set handle
```

Returns true if the provided value is a set handle.

### Parameters

The set handle.

### Return Value

True if the provided value is a set handle.

### Examples

```sh
handle = set_new 1 2 3

value = is_set ${handle}
assert ${value}

released = release ${handle}
assert ${released}
```
