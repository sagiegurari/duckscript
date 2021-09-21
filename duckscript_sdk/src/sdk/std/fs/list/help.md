```sh
var = ls [flags] [path]
```

Lists the file/directory contents.<br>
If no path is provided, the current working directory will be used.<br>
The supported flags are:

* -l - Shows extended information

### Parameters

* Optional flags (currently only -l is supported)
* Optional path (if not provided, current working directory is used)

### Return Value

**true** is operation was successful.

### Examples

```sh
# prints current directory content
ls

# prints current directory content
ls .

# prints examples directory content
ls ./examples

# prints examples directory content with extended info
ls -l ./examples

# prints current directory content with extended info
ls -l

# prints file name
ls ./examples/ls.ds

# prints file name with extended info
ls -l ./examples/ls.ds
```
