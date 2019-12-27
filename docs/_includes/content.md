
<a name="overview"></a>
## Overview
The duckscript task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command, script, rust code or other sub tasks to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, generate documentation, run bench tests, run security validations and more, executed by running a single command.

<a name="installation"></a>
## Installation
In order to install, just run the following command

```sh
cargo install --force duckscript
```

This will install duckscript in your ~/.cargo/bin.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.<br>
<br>
You will have two executables available: *duckscript* and *makers*<br>

* **duckscript** - This is a cargo plugin invoked using ```cargo make ...```
* **makers** - A standalone executable which provides same features and cli arguments as duckscript but is invoked directly and not as a cargo plugin.

<a name="installation-binary-release"></a>
### Binary Release
Binary releases are available in the [github releases page](https://github.com/sagiegurari/duckscript/releases).<br>
The following binaries are available for each release:

* x86_64-unknown-linux-musl
* x86_64-apple-darwin
* x86_64-pc-windows-msvc

Linux builds for arm are available on [bintray](https://bintray.com/sagiegurari/duckscript/linux)

## Contributing
See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/duckscript/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
