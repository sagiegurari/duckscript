```sh
var = get_last_error_line
```

In case of any runtime error, this function will return the error line (if available).

#### Parameters

None

#### Return Value

The last error line or none

#### Examples

```sh
# This will trigger an error
assert_fail

line = get_last_error_line
echo Error Line: ${line}
```
