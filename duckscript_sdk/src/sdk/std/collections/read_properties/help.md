```sh
count = read_properties [--prefix key] text
```

Parses the properties (based on java properties format) text and sets them as variables.<br>
This command will also return the count of properties read.<br>
If prefix is provided, all properties read, will be stored as variables with the **prefix.** as their prefix.

### Parameters

* Optional --prefix and the prefix value
* The text to parse.

### Return Value

The properties count.

### Examples

```sh
count = read_properties "a=1\nb=2\na.b.c=3"
assert_eq ${count} 3

assert_eq ${a} 1
assert_eq ${b} 2
assert_eq ${a.b.c} 3

count = read_properties --prefix config a=1\nb=2\na.b.c=3
assert_eq ${count} 3

assert_eq ${config.a} 1
assert_eq ${config.b} 2
assert_eq ${config.a.b.c} 3
```
