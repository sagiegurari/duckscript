```sh
var = map_size handle
```

Returns the map size based on the provided map handle.

#### Parameters

The map handle.

#### Return Value

The map size.

#### Examples

```sh
handle = map

result = map_put ${handle} a 1
result = map_put ${handle} b 2
result = map_put ${handle} c 3

result = map_size ${handle}
assert_eq ${result} 3

release ${handle}
```
