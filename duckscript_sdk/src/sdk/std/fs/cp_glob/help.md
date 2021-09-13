```sh
result = glob_cp source_glob target
```

This command will copy all files that match the given glob.

#### Parameters

* The source glob, for example ./*.txt
* The target path

#### Return Value

The amount of paths (files) copied or false in case of any error.

#### Examples

```sh
count = glob_cp ./**/*.txt ../target
```
