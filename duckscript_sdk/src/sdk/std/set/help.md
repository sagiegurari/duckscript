```sh
var = set arg [or arg]*
```

The set command will simply return the provided argument and set it to the output variable.<br>
In case the argument is falsy it will attempt to provide another value if an 'or' keyword is set.

A value is considered falsy if it is one of the following:

* false (case insensitive)
* 0
* no (case insensitive)
* Empty value

#### Parameters

The argument to set or an 'or' conditional arguments.


#### Return Value

The first truthy value

#### Examples

```sh
# Return simple 'hello' text value
var = set hello

# Return expanded value: 'home: ....'
var = set "home: ${HOME}"

value = set test or false
assert_eq ${value} test

value = set 0 or no or false or NO or FALSE
assert_eq ${value} FALSE
```
