module.exports = {
  root: true,
  env: {
    node: true,
    browser: true,
    'vue/setup-compiler-macros': true,
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
  plugins: ['@typescript-eslint'],
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
  },
}