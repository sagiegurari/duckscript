```sh
var = set_is_empty handle
```

Returns true if the provided set handle is an empty set.

### Parameters

The set handle.

### Return Value

True if the provided handle belongs to an empty set.

### Examples

```sh
handle = set
set_put ${handle} value
empty = set_is_empty ${handle}
```
