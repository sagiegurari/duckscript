```sh
var = array_get handle index
```

Returns the element from the array at a given index or none if the index is bigger than the array length.

#### Parameters

* The array handle.
* The element index.

#### Return Value

The element at the given index from the array or none.

#### Examples

```sh
handle = array 1 2 3
element = array_get ${handle} 2
assert_eq ${element} 3
```
