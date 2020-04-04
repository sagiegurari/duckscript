```sh
result = array_set handle index value
```

Updates the array at a given index with the provided value.<br>
If the array is not found or the index is greater than the array size, this command will return false.<br>
Otherwise it will return true.

#### Parameters

* The array handle.
* The element index.
* The element value.

#### Return Value

True if successful.

#### Examples

```sh
arr = array old

element = array_get ${arr} 0
assert_eq ${element} old

result = array_set ${arr} 0 new
assert ${result}

element = array_get ${arr} 0
assert_eq ${element} new
```
