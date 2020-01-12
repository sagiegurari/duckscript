```sh
assert_false value [error message]
```

Used to validate the input is falsy.<br>
If the value is one of the following:

* No output
* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

It is considered falsy.

#### Parameters

* The value to evaluate
* Optional error message

#### Return Value

**true** if falsy.

#### Examples

```sh
# valid conditions
assert_false
assert_false false
assert_false 0
assert_false false "This is my error message"

# error conditions (each one will break the execution)
assert_false ok
assert_false true
assert_false yes

value = set "some text"
assert_false ${value}
```
