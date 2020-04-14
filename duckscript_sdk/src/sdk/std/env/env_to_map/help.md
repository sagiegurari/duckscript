```sh
handle = env_to_map
```

Converts all environment variables to a map and returns the map handle.

#### Parameters

None

#### Return Value

The map handle.

#### Examples

```sh
set_env env_to_map_test test_value

handle = env_to_map

value = map_get ${handle} env_to_map_test
assert_eq ${value} test_value

release ${handle}
```
