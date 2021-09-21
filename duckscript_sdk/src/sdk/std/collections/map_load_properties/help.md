```sh
var = map_load_properties [--prefix prefix] handle text
```

Parsers and loads all properties to the provided map.

### Parameters

* Optional --prefix and the prefix value
* The map handle.
* The properties text.

### Return Value

True if successful.

### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_get ${handle} key
assert_eq ${value} value

release ${handle}
```
