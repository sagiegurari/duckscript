```sh
var = replace text from to
```

Returns new value of text after replacing all from values to the provided to values.

### Parameters

* The full text
* The from text
* The to text

### Return Value

The updated text.

### Examples

```sh
text = set "my large text value with lots of text"
updated = replace ${text} text stuff

assert_eq ${updated} "my large stuff value with lots of stuff"
```
