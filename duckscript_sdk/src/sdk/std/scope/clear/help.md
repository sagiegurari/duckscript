```sh
clear_scope name
```

Clears all variables which are prefixed with the provided name + ::.<br>
For example, if the value provided is **my_scope** all variables that start with **my_scope::** will be removed.

#### Parameters

The scope name.

#### Return Value

None.

#### Examples

```sh
testscope = set true
testscope::1 = set 1
testscope::subscope::1 = set 1

assert_eq ${testscope} true
defined = is_defined testscope::1
assert ${defined}
assert_eq ${testscope::1} 1
defined = is_defined testscope::subscope::1
assert ${defined}
assert_eq ${testscope::subscope::1} 1

clear_scope testscope

assert_eq ${testscope} true

defined = is_defined testscope::1
assert_false ${defined}
defined = is_defined testscope::subscope::1
assert_false ${defined}
```
