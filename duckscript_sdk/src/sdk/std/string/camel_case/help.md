```sh
var = camel_case text
```

Converts the provided string into camel case.
All non-alphanumeric characters are ignored.

#### Parameters

The string to convert.

#### Return Value

The converted string.

#### Examples

```sh
string = camel_case "hello, world!"
assert_eq ${string} "HelloWorld"
```
