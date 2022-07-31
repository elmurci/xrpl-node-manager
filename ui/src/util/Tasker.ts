



import ws from "@/util/WsClient";

export default class Tasker {

  constructor(endpoint: string) {
    const intervalId = setInterval(function() {
        console.log("Interval reached every 5s");
        ws.send('pepe');
    }, 5000);
  }
}
