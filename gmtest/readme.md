# use gmssl library

## without install

link library: `./build/lib`
include path: `./include`
run time link: `LD_LIBRARY_PATH=../build/lib:$LD_LIBRARY_PATH ./a.out`

## after installing

link library path: `/usr/local/lib`

1. add it to the system path: `sudo vim /etc/ld.so.conf` or add a file in `/etc/ld.so.conf.d` with a line of path to the library `/usr/local/lib`
2. load config: `sudo ldconfig`
3. test by running `gmssl` command