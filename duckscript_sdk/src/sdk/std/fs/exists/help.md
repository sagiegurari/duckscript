```sh
var = is_path_exists path
```

This command will return true/false based if the provided path points to an existing file system entry.

### Parameters

The path to check.

### Return Value

True if the path points to an existing file system entry.

### Examples

```sh
existing = is_path_exists ./dir
existing = is_path_exists ./dir/somefile.txt
```
