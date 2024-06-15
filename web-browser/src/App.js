import React from 'react';
import init, { create_sql_schema_from_csv, } from '../lib/compile_time_tools';

export default class App extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      worker: new Worker(new URL('worker.js', import.meta.url)),
      loading: true
    };

    this.state.worker.onmessage = (event) => {
      const filename = event.data[0];

      const fileBytes = new Uint8Array(event.data[1]);

      console.log(create_sql_schema_from_csv(filename, fileBytes));
    };
  }

  componentDidMount() {
    init().then(() => this.setState({ loading: false }));
  }

  onFileInputChange(event) {
    const file = event.target.files[0];

    if (file) {
      this.state.worker.postMessage([file]);
    }
  }

  render() {
    const elements = [];

    if (this.state.loading) {
      elements.push((<div key="1">Loading...</div>));
    } else {
      elements.push(
        (
          <input
            key="1"
            type="file"
            accept="text/csv"
            onChange={(event) => this.onFileInputChange(event)}
          />
        )
      );
    }

    return (
      <div>
        <h1>React App</h1>

        { elements }
      </div>
    );
  }
}

