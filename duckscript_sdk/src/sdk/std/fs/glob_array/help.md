```sh
handle = glob_array pattern
```

Returns an array handle containing all path entries found from the provided glob pattern.<br>
The pattern can be a relative path from current directory or an absolute path.

#### Parameters

The glob pattern.

#### Return Value

The array handle.

#### Examples

```sh
handle = glob_array ./somedir/**/*.txt

for path in ${handle}
    echo ${path}
end
```
