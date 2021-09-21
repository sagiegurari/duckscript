```sh
output = semver_is_equal value1 value2
```

Returns true if both semver values are valid and equal.

### Parameters

Two semver values to compare.

### Return Value

True if both semver values are valid and equal, else false.

### Examples

```sh
equal = semver_is_equal 1.2.3 1.2.3
assert ${equal}

equal = semver_is_equal 1.2.3 2.2.3
assert_false ${equal}
```
