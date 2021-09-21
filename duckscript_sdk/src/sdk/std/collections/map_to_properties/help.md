```sh
text = map_to_properties [--prefix prefix] handle
```

Converts the provided map to properties text.

### Parameters

* Optional --prefix and the prefix value
* The map handle.

### Return Value

The properties text.

### Examples

```sh
handle = map
map_put ${handle} a 1
map_put ${handle} b 2
map_put ${handle} a.b.c 123

text = map_to_properties ${handle}
```
