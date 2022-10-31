const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");

let config = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/sync",
      },
    ],
  },
  experiments: {
    syncWebAssembly: true,
  },
  mode: "development",
  plugins: [
    new CopyPlugin({
      patterns: [
        "index.html",
        "favicon.ico",
        { from: "texture", to: "texture" },
      ],
    }),
  ],
};

module.exports = (env, argv) => {
  if (argv.mode === "production") {
    config.mode = "production";
  }

  return config;
};
