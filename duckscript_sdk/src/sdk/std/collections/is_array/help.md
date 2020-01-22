```sh
var = array_length handle
```

Returns the array length based on the provided array handle.

#### Parameters

The array handle.

#### Return Value

The array length.

#### Examples

```sh
handle = array a b c "d e"
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}

handle = range 0 10
len = array_length ${handle}
released = release ${handle}
echo Array length: ${len} released: ${released}
```
