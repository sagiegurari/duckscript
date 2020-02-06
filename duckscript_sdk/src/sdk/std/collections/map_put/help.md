```sh
var = map_put handle key value
```

Inserts a key-value pair into the map.

#### Parameters

* The map handle.
* The key.
* The new value.

#### Return Value

True if a new value was inserted.

#### Examples

```sh
handle = map

result = map_put ${handle} key value
assert_eq ${result} true

value = map_get ${handle} key
assert_eq ${value} value

release ${handle}
```
