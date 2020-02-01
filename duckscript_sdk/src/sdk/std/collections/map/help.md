```sh
handle = map
```

Creates an empty map and returns a handle to that array.<br>
This handle can be passed to other commands which support maps using handles.<br>
Once the map is no longer used, it should be released using the **release** command.

#### Parameters

None

#### Return Value

A handle to the map.

#### Examples

```sh
handle = map

# once done we should release the handle
release ${handle}
```
