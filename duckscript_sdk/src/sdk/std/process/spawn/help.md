```sh
pid = spawn [--silent] command [args]*
```

Executes the provided native command and arguments.<br>
It will not wait for the process to finish and will return the process pid.

#### Parameters

* Option --silent flag to suppress any output.
* The command to execute and its arguments.

#### Return Value

The process pid.

#### Examples

```sh
pid = spawn echo test

echo PID: ${pid}
```
