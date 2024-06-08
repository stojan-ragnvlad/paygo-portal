import React from 'react';
import init, { greet } from 'compile-time-tools';

export default class App extends React.Component {
  componentDidMount() {
    init().then(() => console.log(greet([1, 2, 3])));
  }

  render() {
    return (
      <div>
        <h1>React App</h1>
      </div>
    );
  }
}

