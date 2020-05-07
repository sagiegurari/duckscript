```sh
var = map_contains_key handle key
```

Returns true if the provided key was found in the map.

#### Parameters

* The map handle.
* The key

#### Return Value

True if the key was found in the map.

#### Examples

```sh
handle = map
map_put ${handle} key value
found = map_contains_key ${handle} key
```
