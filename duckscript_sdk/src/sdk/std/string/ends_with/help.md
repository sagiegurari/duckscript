```sh
var = ends_with all partial
```

Returns true if the first argument ends with the value of the second argument.

### Parameters

* The full text to search in
* The suffix text to search for

### Return Value

**true** if ends with.

### Examples

```sh
# valid conditions
result = ends_with abcd abc

value = set "some text"
result = ends_with ${value} "me text"

# will return false
result = ends_with abcd abc
```
