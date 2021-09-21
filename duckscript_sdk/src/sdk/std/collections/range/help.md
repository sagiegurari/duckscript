```sh
handle = range start end
```

Creates an array from the input start and end range values and returns a handle to that array.<br>
This handle can be passed to other commands which support arrays using handles.<br>
Once the array is no longer used, it should be released using the **release** command.

### Parameters

* The start numeric value
* The end numeric value which cannot be smaller than the start value.

### Return Value

A handle to the array.

### Examples

```sh
handle = range 1 10

# once done we should release the handle
release ${handle}
```
