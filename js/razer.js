var events = [];
var messages = [];
Object.keys(window).forEach(key => {
    if (/^on/.test(key)) {
        events.push(key.slice(2))
    }
})

const socket = new WebSocket(location.protocol === 'https:' ? "wss://" : "ws://" + window.location.hostname + ":2794");

for (var i = 0; i < events.length; i++) {
    window.addEventListener(events[i], function(event) {
        var send = "{\"event_name\":\"" + event.type + "\",\"event\":" + stringify_object(event) + "}";
        try {
            socket.send("{\"event_name\":\"" + event.type + "\",\"event\":" + stringify_object(event) + "}");
        }
        catch(err) {
            messages.push(send);
        }
    })
}

socket.onopen = function() {
    for (var i = 0; i < messages.length; i++) {
        socket.send(messages[i]);
    }
}

socket.onmessage = function(event) {
    let data = JSON.parse(event.data);
    if (data.name === "js") {
        eval(data.data);
    } else if (data.name === "html") {
        document.querySelector("body").innerHTML = data.data;
    } else if (data.name === "eval") {
        var run = data.data;
        socket.send("{\"event_name\":\"eval\",\"event\":\"" + eval(run).replaceAll("\"", "\\\"") + "\",\"data\":\"" + run.replaceAll("\"", "\\\"") + "\"}");
    }
};

function stringify_object(object, depth = 0, max_depth = 2) {
    // change max_depth to see more levels, for a touch event, 2 is good
    if (depth > max_depth)
        return 'Object';

    const obj = {};
    for (let key in object) {
        let value = object[key];
        if (value instanceof Node)
            // specify which properties you want to see from the node
            value = {
                id: value.id
            };
        else if (value instanceof Window)
            value = 'Window';
        else if (value instanceof Object)
            value = stringify_object(value, depth + 1, max_depth);

        obj[key] = value;
    }

    return depth ? obj : JSON.stringify(obj);
}