```sh
handle = array_concat [handle]*
```

Concats all provided arrays and returns a handle to a new array with all items.

#### Parameters

Any number of array handles.

#### Return Value

A handle to the new array.

#### Examples

```sh
input1 = range 1 4
input2 = range 4 6
input3 = range 6 8

# new array will contain values from 1-7
arr = array_concat ${input1} ${input2} ${input3}
```
