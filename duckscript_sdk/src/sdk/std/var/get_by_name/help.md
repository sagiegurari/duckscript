```sh
var = get_by_name name
```

This command returns the variable value based on the given variable name.<br>
It is similar to
```sh
var = set ${name}
```
However, it allows for a dynamic variable name.

### Parameters

The variable name.

### Return Value

The variable value or none if no such variable exists.

### Examples

```sh
var = set test
value = get_by_name var
defined = is_defined value

assert ${defined}
assert_eq ${value} test
```
