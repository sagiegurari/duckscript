```sh
var = trim_end value
```

Returns the provided value with trailing whitespace removed.

#### Parameters

The value to trim.

#### Return Value

The trimmed value. If no input provided, this command will return none.

#### Examples

```sh
# trimmed will now hold "  some  text"
trimmed = trim_end "  some  text   "
```
