const webpack = require('webpack');

module.exports = {
  entry: {
    'app': './src/main.ts'
  },
  output: {
    filename: 'app.js'
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "asset/inline"
      }
    ]
  }
};

