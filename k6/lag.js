import http from 'k6/http';
import { randomString, randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.1.0/index.js';
import { sleep } from 'k6';
import ws from 'k6/ws';

function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

export const options = {
    vus: 5,
    iterations: 5,
};

export default function () {

    const res = ws.connect('ws://localhost:8000/ws', null, function (socket) {

        socket.on('open', function open() {

            console.log("connected");

            let deltas = []
            var messages_received = 0
            var messages_sent = 0
            const start_time = Date.now()

            socket.send("#g perro")

            socket.setInterval(function timeout() {

                if (messages_sent < 100) {
                    socket.send("s perro " + Date.now())
                    messages_sent += 1

                }

            }, 500); //write once every 200ms - 500ms

            socket.on('message', function (message) {
                console.log("recv")
                messages_received += 1
                if (message.trim() !== "OK") {
                    const delta = Date.now() - parseInt(message.trim().replace("\n", "").replace("OK", ""))
                    deltas.push(delta.toString())
                }

            });

            //Close after 10 minutes
            socket.setTimeout(function () {

                console.log("msgs received: "+messages_received.toString())
                console.log("msgs sent: "+messages_sent.toString())
                // socket.send("+ deltas " + deltas.join(","))
                console.log(`Closing the socket`);

                socket.close();

            }, 1000 * 120);

        });
    })

}