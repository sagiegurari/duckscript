```sh
result = map_clear handle
```

Clears the provided map.

### Parameters

The map handle.

### Return Value

True if successful.

### Examples

```sh
handle = map

result = map_put ${handle} a 1

result = map_is_empty ${handle}
assert_false ${result}

result map_clear ${handle}
assert ${result}

result = map_is_empty ${handle}
assert ${result}

release ${handle}
```
