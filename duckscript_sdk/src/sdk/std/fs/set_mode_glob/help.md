```sh
result = glob_chmod mode glob
```

This command will update the mode for the given glob pattern.<br>
**This command is currently only available for unix like systems and will return false for all others such as windows.**

#### Parameters

* The new mode, for example 755
* The path glob

#### Return Value

The amount of path entries affected by the operation or false in case of any error.

#### Examples

```sh
file1 = set ./target/_duckscript_test/glob_chmod/modify1.txt
touch ${file1}
file2 = set ./target/_duckscript_test/glob_chmod/modify2.txt
touch ${file2}

count = glob_chmod 777 ./target/_duckscript_test/glob_chmod/**/*.txt
assert_eq ${count} 2

readonly = is_readonly ${file1}
assert_false ${readonly}
readonly = is_readonly ${file2}
assert_false ${readonly}

count = glob_chmod 444 ./target/_duckscript_test/glob_chmod/**/*.txt
assert_eq ${count} 2

readonly = is_readonly ${file1}
assert ${readonly}
readonly = is_readonly ${file2}
assert ${readonly}
```
