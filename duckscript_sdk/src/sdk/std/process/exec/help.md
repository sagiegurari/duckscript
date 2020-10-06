```sh
exec [--fail-on-error] command [args]*

output = exec command [args]*
stdout = set ${output.stdout}
stderr = set ${output.stderr}
exit_code = set ${output.code}

exit_code = exec --get-exit-code command [args]*
```

Executes the provided native command and arguments.<br>
If no output variable is set, the command output will be flushed to the main out/err stream of the parent process.<br>
In addition, in order to fail the command in case of the child process failed, add the --fail-on-error flag.<br>
If an output variable is set, it will be used as a base variable name from which the command stout, stderr and exit code information can be pulled from.<br>
The actual output variable name will not be modified, instead new variables will be created using that variable name as a baseline:

* *output*.stdout - Will hold the stdout text content.
* *output*.stderr - Will hold the stderr text content.
* *output*.code - Will hold the process exit code.

If an output variable is set and the --get-exit-code flag is provided, the output will only contain the exit code.

#### Parameters

* --fail-on-error - If no output variable is provided, it will cause an error in case the executed process exits with an error exit code.
* --get-exit-code - If an output variable is provided, it will contain the exit code.
* The command to execute and its arguments.

#### Return Value

Optionally a base name to access the process stout, stderr and exit code information.

#### Examples

```sh
# Example of running a command and flushing its output to the parent process.
exec echo hello world

# Example of running a command and storing its output.
output = exec echo hello world

stdout = set ${output.stdout}
stderr = set ${output.stderr}
exit_code = set ${output.code}

echo stdout: ${stdout}
echo stderr: ${stderr}
echo exit code: ${exit_code}
```
