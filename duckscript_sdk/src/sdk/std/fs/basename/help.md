```sh
var = basename path
```

This command will return the last path element of the provided path.<br>
If unable, it will return none.

### Parameters

The path to extract the last element from.

### Return Value

The last path element or none if unsuccessful.

### Examples

```sh
file = basename ./dir/file.txt
```
