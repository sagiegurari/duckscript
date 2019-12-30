```sh
release handle
```

Releases an internal handle stored in the runtime memory.<br>
Certain commands (such as **array**) will create a handle and the variable will only hold a reference to that handle.<br>
In order to release those handles once they are no longer needed, the release command should be used.

#### Parameters

The handle name.

#### Return Value

* true - If a handle was found and removed
* false - If no handle was found

#### Examples

```sh
release ${array_handle}
```
