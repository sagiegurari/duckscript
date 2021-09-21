```sh
result = write_binary_file file handle
```

This command enables to write binary data of the provided binary handle into the requested file.<br>
It will return true/false value based if it was able to write the binary data to the file.

### Parameters

* The target file
* The binary data handle

### Return Value

true/false based if it was able to write the binary data to the file.

### Examples

```sh
handle = string_to_bytes "some text"
result = write_binary_file ./target/tests/data.bin ${handle}
```
