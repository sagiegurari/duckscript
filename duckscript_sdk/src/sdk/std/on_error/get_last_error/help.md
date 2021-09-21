```sh
var = get_last_error
```

In case of any runtime error, this function will return the error message.

### Parameters

None

### Return Value

The last error message or none

### Examples

```sh
# This will trigger an error
assert_fail

error = get_last_error
echo Error Message: ${error}
```
