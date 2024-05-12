const webpack = require('webpack');

module.exports = {
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "asset/inline"
      }
    ]
  }
};

