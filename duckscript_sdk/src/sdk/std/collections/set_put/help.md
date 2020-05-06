```sh
var = set_put handle value
```

Pushes an additional value to an existing set.

#### Parameters

The set handle.

#### Return Value

True if a new value was pushed.

#### Examples

```sh
handle = set_new 1 2 3
set_put ${handle} 4
size = set_size ${handle}
assert_eq ${size} 4
```
