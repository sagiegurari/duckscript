```sh
var = alias command arguments
```

This command enables to define new commands with default arguments.<br>
The new alias can be invoked with additional arguments that will be appended to the default set.

#### Parameters

Any number of arguments which will be added to the already defined arguments set during the aliasing.

#### Return Value

**true** if the alias was created, else **false**.

#### Examples

```sh
# This example creates a new **my_echo** alias that will print the prefix before the requested arguments.
created = alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
created = my_echo hello world
```
