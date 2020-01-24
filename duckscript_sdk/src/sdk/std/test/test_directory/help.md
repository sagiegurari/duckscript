```sh
test_directory directory [pattern]
```

This command can be used to run unit tests written in duckscript.<br>
It will run all duckscript files in the directory tree ending with **test.ds** and for each file, it will run
all functions that start with **test_**.<br>
Each such function is considered as a test and can run any type of code and check itself using assert commands.

#### Parameters

* The root directory of all test files (all files ending with **test.ds** in the directory tree will be checked)
* Optional pattern for the file name or test function to limit invocation of only those tests.

#### Return Value

**true** if successful.

#### Examples

This is an example of a test function:

```sh
function test_set_get_unset
    unset_env TEST_SET_GET_UNSET
    value = get_env TEST_SET_GET_UNSET
    assert_false ${value}

    value = set_env TEST_SET_GET_UNSET "test value"
    assert ${value}
    value = get_env TEST_SET_GET_UNSET
    assert_eq ${value} "test value"
end
```
