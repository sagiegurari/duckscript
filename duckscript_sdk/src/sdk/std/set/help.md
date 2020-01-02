```sh
var = set arg
```

The set command will simply return the provided argument and set it to the output variable.

#### Parameters

Only the first argument will be returned.


#### Return Value

The first command argument.

#### Examples

```sh
# Return simple 'hello' text value
var = set hello

# Return expanded value: 'home: ....'
var = set "home: ${HOME}"
```
