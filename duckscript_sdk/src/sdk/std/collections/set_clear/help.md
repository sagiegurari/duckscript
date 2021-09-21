```sh
result = set_clear handle
```

Clears the provided set.

### Parameters

The set handle.

### Return Value

True if successful.

### Examples

```sh
handle = set

result = set_put ${handle} 1

result = set_is_empty ${handle}
assert_false ${result}

result set_clear ${handle}
assert ${result}

result = set_is_empty ${handle}
assert ${result}

release ${handle}
```
