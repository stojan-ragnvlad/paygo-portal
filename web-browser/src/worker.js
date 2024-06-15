onmessage = function (event) {
  const reader = new FileReader();

  reader.readAsArrayBuffer(event.data[0]);

  reader.onload = (loadEvent) => postMessage(reader.result);

  reader.onerror = (error) => postMessage(error);
}

