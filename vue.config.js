const { defineConfig } = require('@vue/cli-service')
const AutoImport = require('unplugin-auto-import/webpack')
const Components = require('unplugin-vue-components/webpack')
const { ElementPlusResolver } = require('unplugin-vue-components/resolvers')

module.exports = defineConfig({
  transpileDependencies: true,
  lintOnSave: false,
  publicPath: './',
  configureWebpack: {
    plugins: [
      AutoImport({
        resolvers: [ElementPlusResolver()],
      }),
      Components({
        resolvers: [ElementPlusResolver()],
      }),
    ],
    resolve: {
      alias: {
        '@': require('path').resolve(__dirname, 'src'),
      },
      extensions: ['.js', '.ts', '.vue', '.json'],
    },
    optimization: {
      splitChunks: {
        chunks: 'all',
        minSize: 20000,
        minRemainingSize: 0,
        minChunks: 1,
        maxAsyncRequests: 30,
        maxInitialRequests: 30,
        enforceSizeThreshold: 50000,
        cacheGroups: {
          vendors: {
            test: /[\\/]node_modules[\\/]/,
            priority: -10,
            reuseExistingChunk: true,
          },
          echarts: {
            test: /[\\/]node_modules[\\/](echarts)[\\/]/,
            name: 'chunk-echarts',
            priority: 10,
            chunks: 'all',
          },
          elementPlus: {
            test: /[\\/]node_modules[\\/](element-plus|@element-plus)[\\/]/,
            name: 'chunk-element-plus',
            priority: 10,
            chunks: 'all',
          },
          default: {
            minChunks: 2,
            priority: -20,
            reuseExistingChunk: true,
          },
        },
      },
    },
    performance: {
      hints: 'warning',
      maxAssetSize: 2048000,
      maxEntrypointSize: 2048000,
    },
  },
  chainWebpack: (config) => {
    config.module
      .rule('ts')
      .test(/\.ts$/)
      .exclude.add(/node_modules/)
      .end()
      .use('ts-loader')
      .loader('ts-loader')
      .options({
        transpileOnly: true,
        appendTsSuffixTo: [/\.vue$/],
      })
      .end()
  },
})
