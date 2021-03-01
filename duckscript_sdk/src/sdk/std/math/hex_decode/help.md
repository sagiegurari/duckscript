```sh
num = hex_decode str
```

Decode a hexadecimal string to the corresponding integer number.<br>
No support for negative numbers.

#### Parameters

A hexadecimal string.

#### Return Value

The corresponding integer number.

#### Examples

```sh
hex_num = set 0xff
num = hex_decode ${hex_num}
res = calc ${num} + 1

assert_eq ${res} 256
```
