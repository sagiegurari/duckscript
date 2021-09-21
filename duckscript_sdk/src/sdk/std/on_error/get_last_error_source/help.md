```sh
var = get_last_error_source
```

In case of any runtime error, this function will return the error source (such as file name) if available.

### Parameters

None

### Return Value

The last error source or none

### Examples

```sh
# This will trigger an error
assert_fail

source = get_last_error_source
echo Error Source File: ${source}
```
