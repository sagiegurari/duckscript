```sh
handle = unset_all_vars [--prefix value]
```

Removes all known variables.<br>
If the prefix is provided, only variables starting with the prefix value will be removed.

### Parameters

* Optional variable name prefix

### Return Value

None

### Examples

```sh
fn test_remove_all
    a = set 1
    b = set 2

    defined = is_defined a
    assert ${defined}
    defined = is_defined b
    assert ${defined}

    unset_all_vars

    defined = is_defined a
    assert_false ${defined}
    defined = is_defined b
    assert_false ${defined}
end

fn test_remove_by_prefix
    root1 = set true
    root1.child = set true
    root12 = set true

    root2 = set true

    defined = is_defined root1
    assert ${defined}
    defined = is_defined root1.child
    assert ${defined}
    defined = is_defined root12
    assert ${defined}
    defined = is_defined root2
    assert ${defined}

    unset_all_vars --prefix root1

    defined = is_defined root1
    assert_false ${defined}
    defined = is_defined root1.child
    assert_false ${defined}
    defined = is_defined root12
    assert_false ${defined}
    defined = is_defined root2
    assert ${defined}
end
```
