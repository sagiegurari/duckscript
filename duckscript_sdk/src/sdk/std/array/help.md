```sh
handle = array value1 value2 value3 ...
```

Creates an array from the input arguments and returns a handle to that array.<br>
This handle can be passed to other commands which support arrays using handles.<br>
Once the array is no longer used, it should be released using the **release** command.

#### Parameters

Any number of arguments which will construct the array.

#### Return Value

A handle to the array.

#### Examples

```sh
handle = array ${var} "hello world" 5 ${another_var}

# once done we should release the handle
release ${handle}
```
