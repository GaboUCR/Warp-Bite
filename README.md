# Warp-Bite

WebSocket proxy for [Bite](https://github.com/alvivar/bite) using [Warp](https://github.com/seanmonstar/warp).

The idea is to use Warp security features as a layer on top of [Bite](https://github.com/alvivar/bite).

_Work in progress!_

## How to profile on Cachegrind (Ubuntu 20.04)

Build the executable with Cargo.

```
cargo build
```

Generate a callgrind.out file with valgrind.

```
valgrind --tool=callgrind --collect-jumps=yes --simulate-cache=yes target/debug/Warp-Bite
```

Run any test you want and after they finished close Warp-Bite, there will be a file called callgrind.out.(int)

Get the number of instructions for each line of code.

```
callgrind_annotate --auto=yes callgrind.out.(int) > profiling-results.txt
```


