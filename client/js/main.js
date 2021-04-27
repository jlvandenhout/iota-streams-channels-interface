const url = 'ws://' + location.host + '/data';
var socket = null;
var interval = 5000;

const output = document.createElement("div");
document.body.appendChild(output);

const input = document.createElement("input");
input.type = "text";
input.value = "";
input.placeholder = "Data...";
document.body.appendChild(input);

const send = document.createElement("button");
send.textContent = "Send";
send.disabled = true;
document.body.appendChild(send);

send.onclick = function() {
    socket.send(input.value);
    input.value = "";
};

function append(data) {
    const line = document.createElement('p');
    line.textContent = data;
    output.appendChild(line);
}

function connect() {
    if (!socket || socket.readyState === WebSocket.CLOSED) {
        socket = new WebSocket(url);

        socket.onopen = function() {
            send.disabled = false;
        };

        socket.onmessage = function(event) {
            append(event.data);
        };

        socket.onclose = function(event) {
            send.disabled = true;
        };
    }
}

connect();
setInterval(connect, interval);
