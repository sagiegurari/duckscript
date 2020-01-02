```sh
assert value [error message]
```

Used to validate the input is truthy.<br>
If the value is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy and will exist with an error.

#### Parameters

* The value to evaluate
* Optional error message

#### Return Value

**true** if truthy.

#### Examples

```sh
# valid conditions
assert ok
assert true
assert yes

value = set "some text"
assert ${value}

# error conditions
assert
assert false
assert 0
assert false "This is my error message"
```
