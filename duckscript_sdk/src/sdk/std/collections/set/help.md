```sh
handle = set_new value1 value2 value3 ...
```

Creates a new set from the input arguments and returns a handle to that set.<br>
This handle can be passed to other commands which support sets using handles.<br>
Once the set is no longer used, it should be released using the **release** command.

#### Parameters

Any number of arguments which will construct the set.

#### Return Value

A handle to the set.

#### Examples

```sh
handle = set_new ${var} "hello world" 5 ${another_var}

# once done we should release the handle
release ${handle}
```
