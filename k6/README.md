# How to run a test
Run [Warp-Bite](https://github.com/GaboUCR/Warp-Bite) and [Bite](https://github.com/alvivar/bite). 

Install [K6](https://k6.io/docs/getting-started/installation/).

Follow the instructions of the test you want to run.

Every test runs 1000 scripts in parallel. 

## Tests

### general-test 
Attempts to test every feature of rust in a random simulation.

```
k6 run general-test.js
```

### chat.js 
Subscribes all clients to one channel. Clients write a message in a range of 200ms to 500ms.

```
k6 run chat.js
```

