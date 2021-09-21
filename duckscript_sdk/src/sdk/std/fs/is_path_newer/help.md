```sh
var = is_path_newer newer older
```

This command will return true if the 'newer' path last modified time is after the 'older' path last modified time.

### Parameters

* newer - The file/directory path to check.
* older - The file/directory path to check.

### Return Value

True if the 'newer' path last modified time is after the 'older' path last modified time.
Otherwise or in case of an error, false will be returned.

### Examples

```sh
newer = is_path_newer ./new_file.txt ./old_file.txt
```
