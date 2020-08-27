```sh
output = semver_is_newer newer older
```

Returns true if both semver values are valid and first value is newer.

#### Parameters

* The expected newer value
* The expected older value

#### Return Value

True if both semver values are valid and first value is newer, else false.

#### Examples

```sh
newer = semver_is_newer 3.2.3 2.2.3
assert ${newer}

newer = semver_is_newer 1.2.3 2.2.3
assert_false ${newer}

newer = semver_is_newer 1.2.3 1.2.3
assert_false ${newer}
```
