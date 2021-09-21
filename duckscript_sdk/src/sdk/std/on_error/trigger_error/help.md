```sh
trigger_error [message]
```

Triggers an error that will trigger the on_error flow.

### Parameters

Optional error message.

### Return Value

None

### Examples

```sh
trigger_error "my error message"
error = get_last_error
assert_eq ${error} "my error message"
```
