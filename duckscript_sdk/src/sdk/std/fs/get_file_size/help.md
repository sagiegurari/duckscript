```sh
var = get_file_size path
```

This command will return the size of the file in bytes.

### Parameters

The path to check.

### Return Value

The size of the file in bytes or false in case path is a directory or does not exist.

### Examples

```sh
size = get_file_size ./dir/somefile.txt
```
