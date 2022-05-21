const events = ["abort","afterprint","animationend","animationiteration","animationstart","beforeprint","beforeunload","blur","canplay","canplaythrough","change","click","contextmenu","copy","cut","dblclick","drag","dragend","dragenter","dragleave","dragover","dragstart","drop","durationchange","ended","error","focus","focusin","focusout","fullscreenchange","fullscreenerror","hashchange","input","invalid","keydown","keypress","keyup","load","loadeddata","loadedmetadata","loadstart","message","mousedown","mouseenter","mouseleave","mousemove","mouseover","mouseout","mouseup","mousewheel","offline","online","open","pagehide","pageshow","paste","pause","play","playing","popstate","progress","ratechange","resize","reset","scroll","search","seeked","seeking","select","show","stalled","storage","submit","suspend","timeupdate","toggle","touchcancel","touchend","touchmove","touchstart","transitionend","unload","volumechange","waiting","wheel"];
let socket = new WebSocket("wss://"+window.location.hostname+"/razer");
for(var i=0;i<events.length;i++) {
    document.addEventListener(events[i], function(event) {
        socket.send(JSON.stringify({event_name:events[i],event:event}));
    })
}
socket.onmessage = function(event) {
    let data = JSON.parse(event.data);
    if (data.name === "js") {
        eval(data.data);
    } else if (data.name === "html") {
        document.querySelector("body").innerHTML= data.data;
    }
};