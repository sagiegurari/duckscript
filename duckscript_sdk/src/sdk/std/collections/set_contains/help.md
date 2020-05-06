```sh
var = set_contains handle value
```

Returns true if the set contains the provided value.

#### Parameters

* The set handle.
* The value

#### Return Value

True if the value was found in the set.

#### Examples

```sh
handle = set_new value1 value2 value3
found = set_contains ${handle} value2
```
