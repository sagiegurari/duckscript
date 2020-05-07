```sh
var = set_size handle
```

Returns the set size based on the provided set handle.

#### Parameters

The set handle.

#### Return Value

The set size.

#### Examples

```sh
handle = set

result = set_put ${handle} 1
result = set_put ${handle} 2
result = set_put ${handle} 3

result = set_size ${handle}
assert_eq ${result} 3

release ${handle}
```
