```sh
value = map_get handle key
```

Returns a the value corresponding to the key from the map.

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

value = map_get ${handle} key
assert_eq ${value} value

release ${handle}
```
