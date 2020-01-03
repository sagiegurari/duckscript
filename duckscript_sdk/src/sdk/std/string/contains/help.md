```sh
var = contains all partial
```

Returns true if the first argument contains the value of the second argument.

#### Parameters

* The full text to search in
* The text to search for

#### Return Value

**true** if contains.

#### Examples

```sh
# valid conditions
result = contains abcd bc

value = set "some text"
result = contains ${value} "me tex"

# will return false
result = contains abcd b1c
```
