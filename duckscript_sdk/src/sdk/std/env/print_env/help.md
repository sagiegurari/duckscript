```sh
var = printenv
```

Prints and returns all environment variables.

### Parameters

None

### Return Value

All environment variables printout text.

### Examples

```sh
set_env TEST_PRINT_ENV TRUE

text = printenv

valid = contains ${text} TEST_PRINT_ENV=TRUE
assert ${valid}
```
