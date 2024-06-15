onmessage = function (event) {
  const reader = new FileReader();

  reader.readAsArrayBuffer(event.data[0]);

  reader.onload =
    (loadEvent) => postMessage([event.data[0].name, reader.result]);

  reader.onerror = (error) => postMessage(error);
}

