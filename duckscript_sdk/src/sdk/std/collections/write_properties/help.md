```sh
text = write_properties [names]
```

Creates a properties string from the provided list of variable names (not values).

#### Parameters

A list of variable names.

#### Return Value

The properties text value.

#### Examples

```sh
a = set 1
b = set 2
a.b.c = set 3

# text will be equal to:
# a=1
# b=2
# a.b.c=3
text = write_properties a b a.b.c
```
