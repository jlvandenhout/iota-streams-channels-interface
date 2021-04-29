const parent = document.body;
const url = "ws://" + location.host + "/data";
const interval = 2000;
var socket = null;

const output = document.createElement("div");
parent.appendChild(output);

const input = document.createElement("input");
input.type = "text";
input.value = "";
input.autofocus = true;
parent.appendChild(input);

function onKeyDown(event) {
    if(event.key === "Enter" && !event.repeat) {
        socket.send(input.value);
        input.value = "";
    }
}

input.addEventListener("keydown", onKeyDown, false);

function onOpen() {
    input.disabled = false;
}

function onMessage(event) {
    const line = document.createElement("p");
    line.textContent = event.data;
    output.appendChild(line);
}

function onClose() {
    input.disabled = true;
}

function connect() {
    if (!socket || socket.readyState === WebSocket.CLOSED) {
        socket = new WebSocket(url);
        socket.addEventListener("open", onOpen);
        socket.addEventListener("message", onMessage);
        socket.addEventListener("close", onClose);
    }
}

connect();
setInterval(connect, interval);