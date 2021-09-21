```sh
var = map_contains_value handle value
```

Returns true if the provided value was found in the map.

### Parameters

* The map handle.
* The value

### Return Value

True if the value was found in the map.

### Examples

```sh
handle = map
map_put ${handle} key value
found = map_contains_value ${handle} value
```
