```sh
scope_pop_stack [--copy name1 name2 ...]
```

Removes all known variables except for the variables provided by the optional --copy argument and than restores the
previously pushed stack.<br>
Functions with the **<scope>** annotation will automatically invoke this command when they end or return a value.

### Parameters

Optional variable names to keep.

### Return Value

None.

### Examples

```sh
var1 = set 1
var2 = set 2

scope_push_stack --copy var2

defined = is_defined var1
echo ${defined}
defined = is_defined var2
echo ${defined}

var3 = set 3
var4 = set 4

scope_pop_stack --copy var4

defined = is_defined var1
echo ${defined}
defined = is_defined var2
echo ${defined}
defined = is_defined var3
echo ${defined}
defined = is_defined var4
echo ${defined}
```
