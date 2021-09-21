```sh
str = hex_encode num
```

Converts an integer number to the corresponding hexadecimal string.<br>
No support for negative numbers.

### Parameters

An integer number.

### Return Value

The corresponding hexadecimal string.

### Examples

```sh
str = hex_encode 255

assert_eq ${str} 0xff
```
