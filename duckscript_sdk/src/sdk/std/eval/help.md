```sh
eval command arguments
```

The eval command enables to run dynamically created commands.<br>
The command and arguments passed can be variables in the form of ${name}.

### Parameters

Any number of arguments which will construct a line to evaluate and execute.

### Return Value

The result of the evaluated line.

### Examples

```sh
command = set echo
eval ${command} hello world
```
