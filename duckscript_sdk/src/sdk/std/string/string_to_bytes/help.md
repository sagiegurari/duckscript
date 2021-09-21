```sh
handle = string_to_bytes text
```

Converts the provided string into binary format and returns a handle to the binary data.

### Parameters

The text to convert.

### Return Value

A handle to the binary data.

### Examples

```sh
handle = string_to_bytes "hello world"
text = bytes_to_string ${handle}

release ${handle}

assert_eq ${text} "hello world"
```
