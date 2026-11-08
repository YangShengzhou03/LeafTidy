module.exports = {
  root: true,
  env: {
    node: true,
    browser: true,
    'vue/setup-compiler-macros': true,
  },
  globals: {
    // Vue 3.3+ 编译器宏，eslint 不识别
    defineOptions: 'readonly',
  },
  extends: [
    'plugin:vue/vue3-essential',
    'eslint:recommended',
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    parser: '@typescript-eslint/parser',
  },
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    // TS 类型声明中的参数名（如 (data: T) => void）不算未使用；插件未安装，关闭参数检查
    'no-unused-vars': ['error', { args: 'none' }],
  },
}