```sh
assert_eq value1 value2 [error message]
```

Used to validate the input is the same.<br>
If they are not, the command will exist with an error.

### Parameters

* Two values to evaluate if they are equal
* Optional error message

### Return Value

**true** if equal.

### Examples

```sh
# valid conditions
assert_eq yes yes
assert_eq false false

value = set "some text"
assert_eq ${value} "some text"

# error conditions (each one will break the execution)
assert_eq 1 2
assert_eq 1 2 "This is my error message"
```
