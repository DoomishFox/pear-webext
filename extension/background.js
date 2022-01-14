console.log("starting pear...");

/*
On startup, connect to the "ping_pong" app.
*/
var port = browser.runtime.connectNative("pear_ff_shim");

/*
Listen for messages from the app.
*/
port.onMessage.addListener((response) => {
  console.log("Received: " + response);
  console.log(response);
});

/*
On a click on the browser action, send the app a message.
*/
browser.browserAction.onClicked.addListener(() => {
  console.log("Sending: echo");
  let packet = { action: 1, payload: {} }
  port.postMessage(packet);
});