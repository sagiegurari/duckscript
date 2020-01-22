```sh
var = array_pop handle
```

Returns the last element of the array or none if the array is empty.

#### Parameters

The array handle.

#### Return Value

The last element of the array or none if the array is empty.

#### Examples

```sh
handle = array 1 2 3
last_element = array_pop ${handle}
assert_eq ${last_element} 3
```
