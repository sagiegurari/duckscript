```sh
var = wget [--method=HTTP-method] [--post-data=payload] [-O file] URL
```

Invokes a HTTP request.<br>
The request method by default is GET but can be modified by the ```--method``` parameter.<br>
The ```-O``` parameter will redirect a valid response output to the provided file, otherwise all response text will be set to the
output variable.<br>
When redirecting to file, the output would be the response size.<br>
The ```--post-data``` parameter enables to pass a payload to POST http requests.<br>
In case of errors or error HTTP response codes, false will be returned.

### Parameters

* Optional HTTP Method, for example --method=HTTP-GET or --method=HTTP-POST (currently only GET and POST are supported).
* Optional post payload via ```--post-data``` parameter.
* Optional redirection of output to file via ```-O``` parameter.
* The target URL

### Return Value

The response text or in case of output redirection to file, the response size.<br>
In case of errors, it will return false.

### Examples

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
