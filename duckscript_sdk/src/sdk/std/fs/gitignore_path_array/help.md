```sh
handle = gitignore_path_array path
```

Returns an array handle containing all path entries found from the provided root path that should be included based on the gitignore definitions.

#### Parameters

The root path.

#### Return Value

The array handle.

#### Examples

```sh
handle = gitignore_path_array ./src

for path in ${handle}
    echo ${path}
end
```
