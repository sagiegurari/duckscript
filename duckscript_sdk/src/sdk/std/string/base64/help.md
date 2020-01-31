```sh
var = base64 [-e] [-encode] [-d] [-decode] value
```

Invokes the base64 encode/decode command with the provided value.<br>
This command allows for a more similar cli command which wraps the base64_encode and base64_decode commands.

#### Parameters

* Optional -e or -encode flags to set the mode to encode (default)
* Optional -d or -decode flags to set the mode to decode
* The value, in case of encoding this is the binary handle, in case of decoding this is the base64 textual value.

#### Return Value

* In case of encoding, the base64 textual value will be returned.
* In case of decoding, a handle to the binary data will be returned.

#### Examples

```sh
function test_get
    response = wget https://www.rust-lang.org/

    found = contains ${response} Rust

    assert ${found}
end

function test_get_to_file
    file = set ./target/_duckscript_test/wget/page.html
    rm ${file}

    response_size = wget -O ${file} https://www.rust-lang.org/

    response = readfile ${file}
    found = contains ${response} Rust

    assert ${found}
    assert ${response_size}
end

function test_post
    payload = set {\"login\":\"login\",\"password\":\"password\"}
    response = wget --method=HTTP-POST --post-data=${payload} https://reqbin.com/echo/post/json

    found = contains ${response} success

    assert ${found}
end
```
