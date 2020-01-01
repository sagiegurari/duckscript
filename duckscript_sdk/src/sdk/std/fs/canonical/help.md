```sh
var = canonicalize path
```

This command will return the c path for the provided input.<br>
In case unable, it will return the original input.

#### Parameters

The file/directory path to canonicalize.

#### Return Value

The canonicalized path, or if unsuccessful, the original path.

#### Examples

```sh
path = canonicalize ./target
```
