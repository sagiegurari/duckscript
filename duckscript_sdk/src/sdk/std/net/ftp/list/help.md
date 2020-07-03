```sh
handle = ftp_list --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>]
```

Invokes the FTP LIST command from the given connection details and path.<br>
Returns a handle to an array of all response entries.

#### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on

#### Return Value

A handle to an array holding all entries.

#### Examples

```sh
handle = ftp_list --host myhost --username someuser --password 12345

for entry in ${handle}
    echo ${entry}
end
```
