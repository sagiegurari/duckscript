```sh
scope_push_stack [--copy name1 name2 ...]
```

Removes all known variables except for the variables provided by the optional --copy argument.<br>
Functions with the **<scope>** annotation will automatically invoke this command and keep only the relevant
function arguments in the new scope.

#### Parameters

Optional variable names to keep.

#### Return Value

None.

#### Examples

```sh
var1 = set 1
var2 = set 2

scope_push_stack --copy var2

defined = is_defined var1
echo ${defined}
defined = is_defined var2
echo ${defined}
```
