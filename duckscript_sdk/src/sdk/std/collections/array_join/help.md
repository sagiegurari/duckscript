```sh
var = array_join handle separator
```

Joins all values in the provided array with the provided separator in between each value.

### Parameters

* An array handle
* The separator to put between each item pair

### Return Value

The joined string value

### Examples

```sh
function test_to_string
    arr = array hello world
    string = array_join ${arr} ", "

    release ${arr}

    assert_eq ${string} "hello, world"
end

function test_numbers
    arr = range 1 5
    string = array_join ${arr} ", "

    release ${arr}

    assert_eq ${string} "1, 2, 3, 4"
end

function test_empty_separator
    arr = range 1 5
    string = array_join ${arr} ""

    release ${arr}

    assert_eq ${string} "1234"
end
```
