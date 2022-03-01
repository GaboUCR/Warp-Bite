import http from 'k6/http';
import { randomString, randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.1.0/index.js';
import { sleep } from 'k6';
import ws from 'k6/ws';

function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

export const options = {
    vus: 500,
    iterations: 500,
};

export default function () {

    const message = randomString(12)

    const res = ws.connect('ws://localhost:8000/ws', null, function (socket) {

        socket.on('open', function open() {

            console.log("connected");

            socket.send("#g perro")
            let i = 0

            socket.setInterval(function timeout () {

                socket.send("s perro " + message)

            }, 1000)

            socket.on('message', function (message) {
                i += 1
                console.log(i)
                // console.log("new message: \n" + message)

            });

            socket.on('close', function () {
                console.log("disconnected");
            });

            //Close after 10 minutes
            socket.setTimeout(function () {

                console.log(`Closing the socket`);

                socket.close();

            }, 1000 * 120);

        });
    })

}
