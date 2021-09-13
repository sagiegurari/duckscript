```sh
var = set_env (key value | --handle map_handle)
```

Sets the environment variable defined by the provided key to the provided value.<br>
If --handle is provided, the second arg will be used as a handle to a map and all keys/values in the map will be set.

#### Parameters

The function can be invoked in the following ways:
* Key/Value pair - Two arguments are required:
  * key - The name of the environment variable to set
  * value - The new environment variable value
* Map handle - Two arguments are required:
  * --handle
  * The map handle

#### Return Value

true if successful

#### Examples

```sh
set_env HOME /usr/me
```
