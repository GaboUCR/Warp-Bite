import http from 'k6/http';
import { randomString, randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.1.0/index.js';
import { sleep } from 'k6';
import ws from 'k6/ws';

function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

export const options = {
    vus: 1000,
    iterations: 1000,
};

export default function () {

    const res = ws.connect('ws://localhost:8000/ws', null, function (socket) {

        socket.on('open', function open() {

            console.log("connected");
            //every clients subscribes to 1000 channels 
            for (let i = 0; i < 1000; i++) {
                socket.send("#b " + i.toString())

            }

            socket.setInterval(function timeout() {

                let rand_channel = getRandomInt(1000)

                let msg_lenght = randomIntBetween(1, 75)
                socket.send("s " + rand_channel.toString() + " " + randomString(msg_lenght))
    
            }, randomIntBetween(200, 500)); //write once every 200ms - 500ms

socket.on('message', function (message) {

    console.log("new message: \n" + message)

});

socket.on('close', function () {
    console.log("disconnected");
});

//Close after 10 minutes
socket.setTimeout(function () {

    console.log(`Closing the socket`);

    socket.close();

}, 1000 * 600);

    });
  })

}
