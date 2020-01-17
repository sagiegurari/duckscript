```sh
value = dump_variables
```

Returns all script variables in textual form.

#### Parameters

None

#### Return Value

The script variables.

#### Examples

```sh
one = set 1
two = set 2
values = array 1 2 yes true
numbers = range -5 15

text = dump_variables
found = contains ${text} two
assert found
found = contains ${text} 2
assert found
found = contains ${text} handle
assert found
```
