```sh
result = join_path path [path]*
```

Concats all paths and makes sure there is a / character between each path element.

#### Parameters

* A list of paths to join

#### Return Value

The joined path

#### Examples

```sh
joined = join_path /test /dir1 /dir2 dir3 //dir4// /dir5

assert_eq ${joined} /test/dir1/dir2/dir3/dir4/dir5
```
