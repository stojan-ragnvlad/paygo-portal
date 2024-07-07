import React from 'react';

export default class App extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      worker: new Worker(new URL('worker.js', import.meta.url))
    };

    this.state.worker.onmessage = (event) => {
      console.log(event.data);

      fetch('/query', { method: 'POST', body: event.data })
        .then((response) => response.json())
        .then((body) => console.log(body));
    };
  }

  onFileInputChange(event) {
    const file = event.target.files[0];

    if (file) {
      this.state.worker.postMessage(file);
    }
  }

  render() {
    return (
      <div>
        <h1>React App</h1>

        <input
          key="1"
          type="file"
          accept="text/csv"
          onChange={(event) => this.onFileInputChange(event)}
        />
      </div>
    );
  }
}

