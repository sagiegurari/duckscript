```sh
var = map_is_empty handle
```

Returns true if the provided map handle is an empty map.

### Parameters

The map handle.

### Return Value

True if the provided handle belongs to an empty map.

### Examples

```sh
handle = map
map_put ${handle} key value
empty = map_is_empty ${handle}
```
