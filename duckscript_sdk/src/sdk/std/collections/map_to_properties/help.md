```sh
var = map_load_properties [--prefix prefix] handle text
```

Parsers and loads all properties to the provided map.

#### Parameters

* Optional --prefix and the prefix value
* The map handle.
* The properties text.

#### Return Value

True if successful.

#### Examples

```sh
handle = map
map_put ${handle} a 1
map_put ${handle} b 2
map_put ${handle} a.b.c 123

text = map_to_properties ${handle}
```
