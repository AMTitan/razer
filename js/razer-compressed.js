var events=[];Object.keys(window).forEach((e=>{/^on/.test(e)&&events.push(e.slice(2))}));let socket=new WebSocket("https:"===location.protocol?"wss://":"ws://"+window.location.hostname+":2794");for(var i=0;i<events.length;i++)window.addEventListener(events[i],(function(e){socket.send('{"event_name":"'+e.type+'","event":'+stringify_object(e)+"}")}));function stringify_object(e,t=0,n=2){if(t>n)return"Object";const o={};for(let a in e){let i=e[a];i instanceof Node?i={id:i.id}:i instanceof Window?i="Window":i instanceof Object&&(i=stringify_object(i,t+1,n)),o[a]=i}return t?o:JSON.stringify(o)}socket.onmessage=function(event){let data=JSON.parse(event.data);"js"===data.name?eval(data.data):"html"===data.name&&(document.querySelector("body").innerHTML=data.data)};
