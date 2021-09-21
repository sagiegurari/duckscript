```sh
var = concat [value]*
```

Concats the provided input into a single string and returns it.

### Parameters

Any number of values to concat.

### Return Value

The result of the concatenation of all input values.

### Examples

```sh
output = concat 1 2 3 4
assert_eq ${output} 1234

output = concat 1 "2 3" 4
assert_eq ${output} "12 34"
```
