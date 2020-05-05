```sh
result = array_clear handle
```

Clears the provided array.

#### Parameters

The array handle.

#### Return Value

True if successful.

#### Examples

```sh
handle = array

result = array_push ${handle} 1

result = array_is_empty ${handle}
assert_false ${result}

result array_clear ${handle}
assert ${result}

result = array_is_empty ${handle}
assert ${result}

release ${handle}
```
