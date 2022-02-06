# How to run a test
Run [Warp-Bite](https://github.com/GaboUCR/Warp-Bite) and [Bite](https://github.com/alvivar/bite). 

To run the test.

```
k6 run --vus x <test-name.js>
```
Where x is the number of asynchronous tests and test-name.js is the test program. K6 only supports Javascript.

## Tests

### General-test 
Attempts to test every feature of rust in a random simulation.

