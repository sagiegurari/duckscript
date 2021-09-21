```sh
var = is_map handle
```

Returns true if the provided value is a map handle.

### Parameters

The map handle.

### Return Value

True if the provided value is a map handle.

### Examples

```sh
map_handle = map

value = is_map ${map_handle}
assert ${value}

released = release ${map_handle}
assert ${released}
```
