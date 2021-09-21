```sh
var = get_last_modified_time path
```

This command will return the last modified time in millies from unix epoch.

### Parameters

The path to check.

### Return Value

The last modified time in millies from unix epoch or false in case path does not exist.

### Examples

```sh
time = get_last_modified_time ./dir/somefile.txt
```
