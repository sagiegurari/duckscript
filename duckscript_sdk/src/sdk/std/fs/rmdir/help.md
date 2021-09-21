```sh
var = rmdir path
```

This command delete the requested empty directory and returns true if successful.<br>
If the path leads to a file or a directory which is not empty, this command will fail.

### Parameters

A single parameter holding the directory path.

### Return Value

**true** if the directory was deleted.

### Examples

```sh
deleted = rmdir ./mydir
```
