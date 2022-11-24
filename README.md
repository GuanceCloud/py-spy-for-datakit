py-spy-for-datakit: Send profiling data to datakit instead of writing to a file
=====

## Installation

Prebuilt binary wheels can be installed from PyPI with:

```
pip install py-spy-for-datakit
```

If you're a Rust user, py-spy can also be installed with: ```cargo install py-spy-for-datakit```.

## Usage

py-spy works from the command line and takes either the PID of the program you want to sample from
or the command line of the python program you want to run.

``` bash
sudo py-spy-for-datakit datakit --host 127.0.0.1 --port 9295 --service py-spy-demo --env dev --version v0.1 --pid 95768
# OR
sudo py-spy-for-datakit datakit --host 127.0.0.1 --port 9295 --service py-spy-demo --env dev --version v0.1 -- python myprogram.py
```

For more usage help, use `py-spy help` and `py-spy help datakit` command or refer to [py-spy](https://github.com/benfred/py-spy) project
