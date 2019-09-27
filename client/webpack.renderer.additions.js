/** @format */

const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = config => {
  // Loader rules
  config.module.rules = [
    {
      test: /\.jsx?$/,
      loader: "babel-loader",
      exclude: /node_modules/,
    },
    {
      test: /\.tsx?$/,
      loader: "ts-loader",
      exclude: /node_modules/,
    },
    {
      test: /\.s?css$/,
      use: [
        MiniCssExtractPlugin.loader,
        "css-loader",
        "postcss-loader",
        "sass-loader",
      ],
    },
    {
      test: /\.(html)$/,
      loader: "html-loader",
    },
  ];

  // Aliases
  config.resolve.alias = {
    ...config.resolve.alias,
    react: "preact/compat",
    "react-dom": "preact/compat",
  };

  // Inferred file extensions
  config.resolve.extensions.push(".jsx");

  // Modules
  config.resolve.modules = [
    path.resolve(__dirname, "src/renderer"),
    path.resolve(__dirname, "node_modules"),
  ];

  // Set up source maps
  config.devtool = "inline-source-map";

  return config;
};
