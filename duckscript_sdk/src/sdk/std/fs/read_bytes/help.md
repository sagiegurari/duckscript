```sh
handle = read_binary_file file
```

Reads a raw file and returns a handle to the binary data.

### Parameters

A single parameter holding the file path.

### Return Value

The binary data handle.

### Examples

```sh
handle = read_binary_file ./Cargo.toml
text = bytes_to_string ${handle}
```
