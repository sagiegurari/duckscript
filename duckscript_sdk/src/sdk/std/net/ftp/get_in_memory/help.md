```sh
handle = ftp_get_in_memory --host <hostname> [--port 21] [--username <user name>] [--password <password>] [--path <path>] [--type <A/I>] --remote-file <file name>
```

Invokes the FTP GET command from the given connection and file details.

### Parameters

* --host - The host name or IP to connect to
* --port - Optional port number to use (by default 21)
* --username - Optional user name used to login (if not user or password provided, no login operation will be invoked)
* --password - Optional password used to login (if not user or password provided, no login operation will be invoked)
* --path - Optional path on the remote server to invoke operation on
* --type - Optional setting of the transfer type as A (ascii) I (image, binary)
* --remote-file - The remote file to download

### Return Value

The binary data handle.

### Examples

```sh
handle = ftp_get_in_memory --host myhost --username someuser --password 12345 --remote-file README.md
text = bytes_to_string ${handle}
```
