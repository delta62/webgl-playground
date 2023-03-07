const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
  },
  mode: "development",
  plugins: [new CopyWebpackPlugin({ patterns: [{ from: "index.html" }] })],
  devtool: "eval-source-map",
  experiments: {
    asyncWebAssembly: true,
  },
};
