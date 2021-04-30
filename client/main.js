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


const node = document.createElement("input");
node.type = "url";
node.name = "node";

const client_role = document.createElement("input");
client_role.id = "client_role"
client_role.value = "client";
client_role.type = "radio";
client_role.name = "role";
const client_label = document.createElement("label");
client_label.for = "client_role";
client_label.innerText = "Client";

const author_role = document.createElement("input");
author_role.id = "author_role"
author_role.value = "author";
author_role.type = "radio";
author_role.name = "role";
const author_label = document.createElement("label");
author_label.for = "author_role";
author_label.innerText = "Author";

const public_type = document.createElement("input");
public_type.id = "public_type";
public_type.value = "public";
public_type.type = "checkbox";
public_type.name = "type";
const public_label = document.createElement("label");
public_label.for = "public_type";
public_label.innerText = "Public";

const masked_type = document.createElement("input");
masked_type.id = "masked_type";
masked_type.value = "masked";
masked_type.type = "checkbox";
masked_type.name = "type";
const masked_label = document.createElement("label");
masked_label.for = "masked_type";
masked_label.innerText = "Masked";

const data = document.createElement("input");
data.type = "textarea";
data.name = "data";

const submit = document.createElement("input");
submit.type = "submit";
submit.innerText = "Submit";

const form = document.createElement("form");
form.appendChild(node);
form.appendChild(client_role);
form.appendChild(client_label);
form.appendChild(author_role);
form.appendChild(author_label);
form.appendChild(public_type);
form.appendChild(public_label);
form.appendChild(masked_type);
form.appendChild(masked_label);
form.appendChild(data);
form.appendChild(submit);

parent.appendChild(form);

async function onSubmit(event) {
    event.preventDefault();

    let formData = new FormData(form);
    let response = await fetch("/form", { method: "POST", body: formData });
    console.log(response);
}

form.addEventListener("submit", onSubmit);