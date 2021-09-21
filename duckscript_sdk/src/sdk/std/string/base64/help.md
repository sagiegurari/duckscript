```sh
var = base64 [-e] [-encode] [-d] [-decode] value
```

Invokes the base64 encode/decode command with the provided value.<br>
This command allows for a more similar cli command which wraps the base64_encode and base64_decode commands.

### Parameters

* Optional -e or -encode flags to set the mode to encode (default)
* Optional -d or -decode flags to set the mode to decode
* The value, in case of encoding this is the binary handle, in case of decoding this is the base64 textual value.

### Return Value

* In case of encoding, the base64 textual value will be returned.
* In case of decoding, a handle to the binary data will be returned.

### Examples

```sh
handle = string_to_bytes "hello world"
text = base64 ${handle}
release ${handle}
assert_eq ${text} aGVsbG8gd29ybGQ=

handle = base64 -decode ${text}
text = bytes_to_string ${handle}
release ${handle}
assert_eq ${text} "hello world"
```
