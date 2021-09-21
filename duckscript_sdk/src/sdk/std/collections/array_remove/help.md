```sh
result = array_remove handle index
```

Removes the item from the array at the given index.<br>
If the array is not found or the index is greater than the array size, this command will return false.<br>
Otherwise it will return true.

### Parameters

* The array handle.
* The element index.

### Return Value

True if successful.

### Examples

```sh
arr = array old

element = array_get ${arr} 0
assert_eq ${element} old

result = array_remove ${arr} 0
assert ${result}

empty = array_is_empty ${arr}
assert ${empty}
```
