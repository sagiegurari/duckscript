```sh
count = read_properties text
```

Parses the properties (based on java properties format) text and sets them as variables.<br>
This command will also return the count of properties read.

#### Parameters

The text to parse.

#### Return Value

The properties count.

#### Examples

```sh
count = read_properties "a=1\nb=2\na.b.c=3"
assert_eq ${count} 3

assert_eq ${a} 1
assert_eq ${b} 2
assert_eq ${a.b.c} 3
```
