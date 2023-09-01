const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "./dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  // resolve: {
  //   extensions: [".png", ".ttf",".js"],
  // },
  // module: {
  //   rules: [
  //     {
  //       test: /\.css$/i,
  //       use: ['style-loader','css-loader']
  //     },
  //     // {
  //     //   test: /\.(png)$/,
  //     //   loader: 'url-loader',
  //     //   options: {
  //     //     //publicPath: './dist/assets/',
  //     //     name: '[name].[ext]', // [name].[ext]?[hash]
  //     //     limit: 5000
  //     //   }
  //     // },
  //     {
  //       test: /\.(ttf)$/i,
  //       loader: 'file-loader',
  //       options: {
  //         publicPath: '/assets/fonts',
  //         name: '[name].[ext]' // [name].[ext]?[hash]
  //       }
  //     }
  //   ]
  // },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
