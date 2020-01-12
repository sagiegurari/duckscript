```sh
var = exit_on_error value
```

Enables to cause the script execution to stop in case of any error.<br>
By default all errors simply trigger the on_error command which the default SDK stores and provides access to.<br>
However, with this command you can change the on_error command to instead stop the script execution.

#### Parameters

If no argument is provided, it will return the current state.<br>
If an argument is provided, it will modify the state and return it as true/false.

#### Return Value

The current/updated state as true/false value

#### Examples

```sh
# Get current state
will_exit = exit_on_error
echo Current state: ${will_exit}

# Update the current state
will_exit = exit_on_error true
echo Current state: ${will_exit}
```
