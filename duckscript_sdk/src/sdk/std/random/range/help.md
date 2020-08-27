```sh
output = random_range min max
```

Generate a random value in the range of min and max values provided, i.e. inclusive of min and exclusive of max.

#### Parameters

* min - The min range value (inclusive)
* max - The max range value (exclusive)

#### Return Value

The generated numeric value.

#### Examples

```sh
value = random_range -10 10
echo ${value}
```
