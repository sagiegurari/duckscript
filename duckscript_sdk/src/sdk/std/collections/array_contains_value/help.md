```sh
var = array_contains_value handle value
```

Returns the first index of the array with the same value as provided.<br>
If not found, false will be returned.

#### Parameters

* The array handle.
* The value

#### Return Value

The value index in the array or false if not found.

#### Examples

```sh
handle = array value1 value2 value3
index = array_contains_value ${handle} value2
```
