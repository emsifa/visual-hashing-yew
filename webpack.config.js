const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');

const NODE_ENV = process.env.NODE_ENV;

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    mode: NODE_ENV || argv.mode || 'development',
    optimization: {
      minimizer: [new TerserPlugin({}), new OptimizeCSSAssetsPlugin({})],
    },
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "app.js",
      webassemblyModuleFilename: "app.wasm"
    },
    module: {
      rules: [
        {
          test: /\.css$/i,
          use: [
            MiniCssExtractPlugin.loader,
            'css-loader',
            'postcss-loader',
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
      new MiniCssExtractPlugin({
        filename: 'style.css',
      }),
    ],
    watch: argv.mode !== 'production'
  };
};
