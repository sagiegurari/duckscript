```sh
var = cp source target
```

This command copies the requested file or directory to the target location.<br>
If the source directory is not empty, its entire contents will be copied as well.

### Parameters

* The source path to copy
* The target path

### Return Value

**true** if the path was copied.

### Examples

```sh
# copy a single file
copied = cp ./file1.txt ./file2.txt

# copy a directory
copied = cp ./source ./target
```
