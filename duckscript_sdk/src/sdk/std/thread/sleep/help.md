```sh
sleep millies
```

Will cause the script execution to half for the given amount of milliseconds.<br>
The command will also return the amount of milliseconds waited.

#### Parameters

A positive numeric value.

#### Return Value

The amount of milliseconds waited.

#### Examples

```sh
# will sleep for 10 milliseconds
time = sleep 10
echo Waited for ${time} milliseconds.
```
