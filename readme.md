# libipt-sys
low level rustgen bindings for the [libipt](https://github.com/intel/libipt) library.

## building

clone this repo and the submodules with
> git clone https://github.com/sum-catnip/winipt-sys.git --recursive

simply run `cargo build` and you should be fine.
If that doesnt work, make sure you got all the requirements covered.

## requirements

- a working c compiler that can be found by cmake
- cmake
- maybe rust nightly (im not even sure tbh)