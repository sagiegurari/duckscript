```sh
var = rm [-r] [path]+
```

This command delete the requested file/s, empty directories or recursively deletes directories
and all their content (files and sub directories) if the **-r** flag is provided.

### Parameters

* Optional flags (currently only -r is supported which indicates recursive deletion)
* The path/s to delete

### Return Value

**true** if all paths were deleted.

### Examples

```sh
# delete a file or empty directory
deleted = rm ./target

# deletes a directory and all its content
deleted = rm -r ./target

# delete all provided paths
deleted = rm -r ./dir ./somefile ./anotherdir/subdir/file
```
