```sh
var = array_push handle value
```

Pushes an additional value to an existing array.

#### Parameters

The array handle.

#### Return Value

True if a new value was pushed.

#### Examples

```sh
handle = array 1 2 3
array_push ${handle} 4
last_element = array_pop ${handle}
assert_eq ${last_element} 4
```
