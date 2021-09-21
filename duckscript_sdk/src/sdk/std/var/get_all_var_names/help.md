```sh
handle = get_all_var_names
```

Creates an array holding all currently known variable names and returns the array handle.

### Parameters

None

### Return Value

A handle to the array.

### Examples

```sh
handle = get_all_var_names

# once done we should release the handle
release ${handle}
```
