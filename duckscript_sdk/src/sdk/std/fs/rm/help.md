```sh
var = rm [-r] path
```

This command delete the requested file, empty directory or recursively deletes a directory
and all its content (files and sub directories) if the **-r** flag is provided.

#### Parameters

* Optional flags (currently only -r is supported which indicates recursive deletion)
* The path to delete

#### Return Value

**true** if the path was deleted.

#### Examples

```sh
# delete a file or empty directory
deleted = rm ./target

# deletes a directory and all its content
deleted = rm -r ./target
```
