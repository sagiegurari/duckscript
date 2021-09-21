```sh
removed = set_remove handle value
```

Removes a the value from the set and returns true/false if it was removed.

### Parameters

* The set handle.
* The value to remove.

### Return Value

True if the value was found and removed from the set.

### Examples

```sh
handle = set_new

result = set_put ${handle} value
assert_eq ${result} true

removed = set_remove ${handle} value
assert ${removed}

release ${handle}
```
