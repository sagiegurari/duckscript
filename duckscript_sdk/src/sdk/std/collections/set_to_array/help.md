```sh
array_handle = set_to_array set_handle
```

Converts the provided set to an array and returns the new array handle.

#### Parameters

The set handle.

#### Return Value

The array handle or false in case of error.

#### Examples

```sh
set_handle = set_new value1 value2 value3
array_handle = set_to_array ${set_handle}
```
