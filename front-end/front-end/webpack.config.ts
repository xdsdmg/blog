/// <reference path="node_modules/webpack-dev-server/types/lib/Server.d.ts"/>

import * as webpack from 'webpack';
import path from 'path';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';

interface BuildEnv {
  analyzer?: boolean;
  [key: string]: any;
}

interface BuildArgv {
  mode?: webpack.Configuration['mode'];
  [key: string]: any;
}

type ConfigFactoryFn = (env: BuildEnv | undefined, argv: BuildArgv | undefined) => webpack.Configuration;

const ROOT_PATH = process.cwd();
const DEV_SERVER_PORT = 5847;
const NODE_MODULE_PATH = path.resolve(ROOT_PATH, './node_modules');

const loadersWithModuleEnabled: webpack.RuleSetUseItem[] = [
  'css-modules-typescript-loader',
  {
    loader: 'css-loader',
    options: {
      modules: {
        localIdentName: '[local]_[hash:base64:8]'
      }
    }
  }
];

const lessLoader: webpack.RuleSetUseItem = {
  loader: 'less-loader',
  options: {
    lessOptions: {
      javascriptEnabled: true,
      modifyVars: {
        prefix: 'sa'
      }
    }
  }
};

const configFactoryFn: ConfigFactoryFn = (env, argv) => {
  const isProdMode = argv?.mode === 'production';
  const cssExtractLoader = isProdMode ? MiniCssExtractPlugin.loader : 'style-loader';

  const config: webpack.Configuration = {
    mode: argv?.mode,
    entry: {
      index: path.join(ROOT_PATH, 'src/index.tsx')
    },
    plugins: [],
    resolve: {
      extensions: ['.ts', '.tsx', '.js', '.json', '.jsx', '.css', '.less', '.json', '.yml'],
      alias: {
        '~pages': path.resolve(__dirname, './src/pages'),
        '~components': path.resolve(__dirname, './src/components'),
        '~models': path.resolve(__dirname, './src/models'),
        '~common': path.resolve(__dirname, './src/common'),
      }
    },
    module: {
      rules: [
        {
          test: /\.css$/,
          oneOf: [
            {
              test: targetPath => targetPath.includes(NODE_MODULE_PATH),
              use: [cssExtractLoader, 'css-loader', 'postcss-loader']
            },
            {
              test: targetPath => !targetPath.includes(NODE_MODULE_PATH),
              use: [cssExtractLoader, ...loadersWithModuleEnabled, 'postcss-loader']
            }
          ]
        },
        {
          test: /\.less$/,
          oneOf: [
            {
              test: targetPath => targetPath.includes(NODE_MODULE_PATH),
              use: [cssExtractLoader, 'css-loader', 'postcss-loader', lessLoader]
            },
            {
              test: targetPath => !targetPath.includes(NODE_MODULE_PATH),
              use: [cssExtractLoader, ...loadersWithModuleEnabled, 'postcss-loader', lessLoader]
            }
          ]
        },
      ]
    }
  };

  if (isProdMode) {
    config.output = {
      clean: true,
      filename: '[name].[chunkhash].js',
      chunkFilename: '[name].[chunkhash].bundle.js',
      publicPath: '/',
      path: path.resolve(__dirname, './dist/')
    };
    config.plugins!.push(
      new MiniCssExtractPlugin({
        filename: '[name].[contenthash].min.css',
        ignoreOrder: true
      }),
      new HtmlWebpackPlugin({
        filename: 'index.html',
        template: './index.html',
        name: 'index',
        inject: true,
        environment: process.env,
        chunks: ['manifest', 'vendor', 'index']
      })
    );
    config.module!.rules!.unshift({
      test: /\.(j|t)sx?$/,
      use: 'babel-loader'
    });
  } else {
    config.output = {
      filename: '[name].js',
      chunkFilename: '[name].bundle.js',
      publicPath: '/',
      path: path.resolve(__dirname, './dist/')
    };
    config.devServer = {
      port: DEV_SERVER_PORT,
      open: true,
      hot: 'only',
      proxy: {
        '/api': {
          target: 'http://localhost:8888',
          changeOrigin: true,
        },
      },
      historyApiFallback: {
        disableDotRule: true
      },
      devMiddleware: {
        writeToDisk: false
      }
    };
    config.plugins!.push(
      new HtmlWebpackPlugin({
        filename: 'index.html',
        template: './index.html',
        name: 'index',
        inject: true,
        environment: process.env,
        chunks: ['manifest', 'vendor', 'index']
      })
    );
    config.module!.rules!.unshift({
      test: /\.(j|t)sx?$/,
      exclude: /node_modules/,
      use: 'babel-loader'
    });
  }

  return config;
}

export default configFactoryFn
