```sh
set_error message
```

Sets the last error which is accessible via get_last_error.<br>
This command will not trigger the on_error command flow.

### Parameters

The error message.

### Return Value

None

### Examples

```sh
set_error "my error message"

error = get_last_error

assert_eq ${error} "my error message"
```
