```sh
assert_error [error message]
```

This command will cause a runtime error which will not stop the script execution.<br>
If error message is provided, it will be used as part of the error output.

#### Parameters

Optional error message.

#### Return Value

None

#### Examples

```sh
assert_error

assert_error "This is my error message"
```
