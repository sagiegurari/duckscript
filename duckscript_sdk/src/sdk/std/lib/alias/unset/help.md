```sh
unalias name
```

Removes previously defined alias and return true/false based if an alias was actually removed.

#### Parameters

The alias name to remove.

#### Return Value

A true/false value in case an alias with the provided name existed.

#### Examples

```sh
alias my_echo echo [ECHO]

# This will print "[ECHO] hello world "
my_echo hello world

unalias my_echo

# This will error
echo The script will now error as my_echo is no longer defined
my_echo hello world
```
