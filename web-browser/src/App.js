import React from 'react';
import init, { create_sql_schema_from_csv, } from '../lib/compile_time_tools';

export default class App extends React.Component {
  componentDidMount() {
    init().then(() => console.log(create_sql_schema_from_csv([1, 2, 3])));
  }

  render() {
    return (
      <div>
        <h1>React App</h1>
      </div>
    );
  }
}

