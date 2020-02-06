```sh
text = bytes_to_string handle
```

Converts the provided UTF-8 binary array to string and returns it.

#### Parameters

A handle to a binary array holding UTF-8 text.

#### Return Value

The textual data.

#### Examples

```sh
handle = string_to_bytes "hello world"
text = bytes_to_string ${handle}

release ${handle}

assert_eq ${text} "hello world"
```
