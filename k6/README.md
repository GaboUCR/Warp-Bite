# How to run a test
Run [Warp-Bite](https://github.com/GaboUCR/Warp-Bite) and [Bite](https://github.com/alvivar/bite). 

Install [K6](https://k6.io/docs/getting-started/installation/).

Follow the instructions of the test you want to run.

## Tests

### General-test 
Attempts to test every feature of rust in a random simulation.

```
k6 run --vus 10 general-test.js
```

This command runs 10 scripts in parallel, change the vus command according to your needs. 
