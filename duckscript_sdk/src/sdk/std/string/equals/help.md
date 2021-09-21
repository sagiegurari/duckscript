```sh
var = eq value1 value2
```

Returns true if both provided values are equal.

### Parameters

Two values to evaluate if they are equal

### Return Value

**true** if equal.

### Examples

```sh
# valid conditions
is_same = eq yes yes
is_same = eq false false

value = set "some text"
is_same = eq ${value} "some text"

# will return false
is_same = eq 1 2
```
