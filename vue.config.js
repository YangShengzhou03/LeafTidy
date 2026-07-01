const { defineConfig } = require('@vue/cli-service')

module.exports = defineConfig({
  transpileDependencies: true,
  lintOnSave: false,
  configureWebpack: {
    resolve: {
      alias: {
        '@': require('path').resolve(__dirname, 'src'),
      },
      extensions: ['.js', '.ts', '.vue', '.json'],
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
