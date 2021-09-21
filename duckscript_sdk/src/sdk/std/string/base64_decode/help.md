```sh
text = base64_encode handle
```

Encodes using base64 the provided binary data and returns the encoded text value.<br>
The binary data is provided as a handle.

### Parameters

The handle to the binary data to encode.

### Return Value

The encoded textual value.

### Examples

```sh
handle = string_to_bytes "hello world"
text = base64_encode ${handle}

release ${handle}

assert_eq ${text} "hello world"
```
