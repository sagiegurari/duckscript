```sh
var = starts_with all partial
```

Returns true if the first argument starts with the value of the second argument.

#### Parameters

* The full text to search in
* The prefix text to search for

#### Return Value

**true** if starts with.

#### Examples

```sh
# valid conditions
result = starts_with abcd abc

value = set "some text"
result = starts_with ${value} "some te"

# will return false
result = starts_with abcd bcd
```
