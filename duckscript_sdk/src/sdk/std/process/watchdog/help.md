```sh
count = watchdog [--max-retries value] [--interval value] -- command [arguments]*
```

Executes the provided native command and arguments.<br>
In case the command existed it will be executed again up to the max retries provided.<br>
The watchdog will wait the specified time in milliseconds between invocations.<br>
In case of an invalid command, the watchdog will not reattempt the invocation and will exit without retries.

#### Parameters

* --max-retries - Positive value of max retries (excluding the first invocation). value <= 0 for unlimited retries. Default is unlimited.
* --interval - The amount in milliseconds between retries. 0 for no waiting between invocations. Default is no wait.
* The command to executed (preceded by a **--** separator).
* The command arguments.

#### Return Value

The amount of invocations or false in case of any error.

#### Examples

```sh
count = watchdog --max-retries 0 -- echo test
assert_eq ${count} 1

count = watchdog --max-retries 3 --interval 10 -- echo test
assert_eq ${count} 4
```
