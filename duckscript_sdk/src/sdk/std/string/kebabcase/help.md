```sh
var = kebobcase text
```

Converts the provided string into kebob case.
All non-alphanumeric characters are ignored.

#### Parameters

The string to convert.

#### Return Value

The converted string.

#### Examples

```sh
string = kebobcase "Hello, World!"
assert_eq ${string} "hello-world"
```

