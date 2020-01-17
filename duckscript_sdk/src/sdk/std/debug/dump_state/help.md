```sh
value = dump_state
```

Returns all script state in textual form.

#### Parameters

None

#### Return Value

The script state.

#### Examples

```sh
numbers = range -5 15

text = dump_instructions
found = contains ${text} -5
assert found
```
