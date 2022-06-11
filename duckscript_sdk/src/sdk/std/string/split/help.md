```sh
handle = split text pattern
```

Splits the provided text based on the provided pattern and return a handle the
created array with all the split values.

### Parameters

* The text to split
* The pattern to split by

### Return Value

A handle to the values array.

### Examples

```sh
handle = split a23b23c23d23e 23

len = array_length ${handle}

value = array_pop ${handle}
assert_eq ${value} e
value = array_pop ${handle}
assert_eq ${value} d
value = array_pop ${handle}
assert_eq ${value} c
value = array_pop ${handle}
assert_eq ${value} b
value = array_pop ${handle}
assert_eq ${value} a

release ${handle}

assert_eq ${len} 5
```
