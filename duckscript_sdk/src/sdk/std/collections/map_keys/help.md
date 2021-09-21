```sh
keys = map_keys handle
```

Returns a handle to an array holding all keys in the provided map handle.

### Parameters

* The map handle.

### Return Value

A handle to an array holding all map keys.

### Examples

```sh
handle = map

result = map_put ${handle} key1 value1
assert_eq ${result} true
result = map_put ${handle} key2 value2
assert_eq ${result} true

keys = map_keys ${handle}
for key in ${keys}
    value = map_get ${handle} ${key}
    echo Key: ${key} Value: ${value}
end

release ${handle}
release ${keys}
```
