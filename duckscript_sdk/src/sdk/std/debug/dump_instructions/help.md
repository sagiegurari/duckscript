```sh
value = dump_instructions
```

Returns all script instructions structure (not script text) in textual form.

### Parameters

None

### Return Value

The script instructions.

### Examples

```sh
value = dump_instructions
found = contains ${value} dump_instructions
assert found
```
