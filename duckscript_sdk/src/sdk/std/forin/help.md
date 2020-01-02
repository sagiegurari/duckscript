```sh
args = array a b c
for arg in args
    # commands
end_for
release args
```

The for/in command enables to iterate over an array (see [array command](#sdk__Array)).<br>
The first argument will contain the current iteration value from the array.<br>
Once all values have been read, it will exit the loop.

#### Parameters

* for
  * The variable name which will hold the current iteration value
  * The string "in"
  * The handle to the array of values to iterate
* end_for - no parameters

#### Return Value

None

#### Examples

```sh
# Simple example iteration over the list of letters:
args = array a b c

for arg in args
    echo current arg is: ${arg}
end_for

release args

# Example nested loops:
range = array 1 2 3
for i in range
    for j in range
        echo i: ${i} j: ${j}
    end_for
end_for
```
