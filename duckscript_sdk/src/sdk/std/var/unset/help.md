```sh
unset [names]*
```

Undefines all the variable names provided.

### Parameters

A list of variable names to undefine.

### Return Value

None

### Examples

```sh
var = set 1
defined = is_defined var
assert ${defined}
unset var
defined = is_defined var
assert_false ${defined}
```
