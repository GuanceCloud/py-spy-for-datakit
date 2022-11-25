py-spy-for-datakit: Send profiling data to datakit instead of writing to a file
=====

## Installation

Prebuilt binary wheels can be installed from PyPI with:

```
pip install py-spy-for-datakit
```

If you have rust and cargo installed, py-spy-for-datakit can also be installed with: ```cargo install py-spy-for-datakit```.

In addition to above, you can also download prebuilt binaries for most platforms from the [GitHub Releases
Page](https://github.com/GuanceCloud/py-spy-for-datakit/releases) directly.

## Usage

py-spy-for-datakit works from the command line and takes either the PID of the program you want to sample from
or the command line of the python program you want to run.

``` bash
# Use process PID
sudo py-spy-for-datakit datakit --host 127.0.0.1 --port 9295 --service py-spy-demo --env dev --version v0.1 --pid 12345
# OR use your app starting command line
sudo py-spy-for-datakit datakit --host 127.0.0.1 --port 9295 --service py-spy-demo --env dev --version v0.1 -- python myapp.py
```

For more usage help, use  `py-spy-for-datakit help datakit` command or refer to [py-spy](https://github.com/benfred/py-spy) manual
