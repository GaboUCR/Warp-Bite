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

      socket.setInterval(function timeout() {
        console.log("principal")
        var rand_msg = getRandomInt(100);
        
        // 5 percent change of subscribing to a new channel

        if (0 <= rand_msg && rand_msg <= 4 ) {
          console.log("sub")
          let rand_channel = getRandomInt(10000)
          socket.send("#b "+ rand_channel.toString())
        }
        
        //95 percent change of writing a new message
        else if ( 5 <= rand_msg && rand_msg <= 99) {
          console.log("msg")
          let rand_channel = getRandomInt(10000)
          let rand_msg_lenght = getRandomInt(100)

          let msg_lenght = randomIntBetween(512, 1024)
          socket.send("s "+rand_channel.toString()+" "+randomString(msg_lenght))

          // //20 percent change of a large message betweem 5 Mb and 500Mb
          // if (0 <= rand_msg_lenght && rand_msg_lenght <= 19) {
          //   console.log("large message")
          //   let msg_lenght = randomIntBetween(1024*1000*5, 1024*1000*500)
          //   socket.send("s "+rand_channel.toString()+" "+randomString(msg_lenght))
          // }
          // //80 percent change of a message between 20b and 200b
          // else if (20 <= rand_msg_lenght && rand_msg_lenght <= 99) {
          //   console.log("small message")
          //   let msg_lenght = randomIntBetween(20, 200)
          //   socket.send("s "+rand_channel.toString()+" "+randomString(msg_lenght))
          // }
        }
      }, randomIntBetween(200, 1000)); 

      socket.on('message', function (message) {

        console.log("new message: \n"+ message)
  
      });

      socket.on('close', function () {
        console.log("disconnected");
      });


      socket.setTimeout(function () {

        console.log(`Closing the socket`);
  
        socket.close();
  
      }, 1000*600);

    });
  })

}
