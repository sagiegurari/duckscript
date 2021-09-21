```sh
value = map_remove handle key
```

Removes a the value corresponding to the key from the map and returns it.

### Parameters

* The map handle.
* The key.

### Return Value

The value corresponding to the key from the map.

### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_remove ${handle} key
assert_eq ${value} value

release ${handle}
```
