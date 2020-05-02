```sh
var = set_by_name name value
```

This command sets the variable value based on the variable name.<br>
It is similar to
```sh
name = set ${value}
```
However, it allows for a dynamic variable name.

#### Parameters

* The variable name.
* The new variable value.

#### Return Value

The new variable value.

#### Examples

```sh
var = set test
value = get_by_name var
defined = is_defined value

assert ${defined}
assert_eq ${value} test
```
