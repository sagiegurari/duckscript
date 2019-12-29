```sh
exec command [args]*

handle = exec command [args]*
stdout = get_stdout ${handle}
stderr = get_stderr ${handle}
exit_code = get_exit_code ${handle}
release handle
```

Executes the provided native command and arguments.<br>
If an output variable is set, it will hold a handle from which the command stout, stderr and exit code information can be pulled from.<br>
If no output variable is set, the command output will be flushed to the main out/err stream of the parent process.

#### Parameters

The command to execute and its arguments.

#### Return Value

Optionally a handle to access the process stout, stderr and exit code information.

#### Examples

Example of running a command and flushing its output to the parent process.

```sh
exec echo hello world
```

Example of running a command and storing its output in the output handle.

```sh
handle = exec echo hello world

stdout = get_stdout ${handle}
stderr = get_stderr ${handle}
exit_code = get_exit_code ${handle}

echo stdout: ${stdout}
echo stderr: ${stderr}
echo exit code: ${exit_code}

release handle
```
