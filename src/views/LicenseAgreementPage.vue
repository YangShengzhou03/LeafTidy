<template>
  <div class="license-page">
    <div class="license-content">
      <h1 class="page-title">许可协议</h1>

      <div class="license-section">
        <div class="section-title">MIT License</div>
        <div class="section-content">
          <p>Copyright (c) 2024 LeafTidy</p>
          <p>Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:</p>
          <p>The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.</p>
          <p>THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.</p>
        </div>
      </div>

      <div class="license-section">
        <div class="section-title">协议说明</div>
        <div class="section-content">
          <p>轻羽归档(LeafTidy)采用 MIT 许可证开源发布。MIT 许可证是一种宽松的开源许可证，允许您：</p>
          <ul>
            <li>自由使用：您可以免费使用本软件，无需支付任何费用</li>
            <li>自由复制：您可以复制本软件，用于个人或商业目的</li>
            <li>自由修改：您可以修改本软件的源代码，以适应您的需求</li>
            <li>自由分发：您可以分发本软件的原始版本或修改版本</li>
            <li>商业使用：您可以在商业项目中使用本软件</li>
          </ul>
        </div>
      </div>

      <div class="license-section">
        <div class="section-title">使用条件</div>
        <div class="section-content">
          <p>在使用本软件时，您需要遵守以下条件：</p>
          <ul>
            <li>保留版权声明：在软件的所有副本中保留原始版权声明和许可声明</li>
            <li>保留许可声明：在软件的所有副本中保留 MIT 许可证文本</li>
          </ul>
        </div>
      </div>

      <div class="license-section">
        <div class="section-title">免责声明</div>
        <div class="section-content">
          <p>本软件按"原样"提供，不附带任何明示或暗示的保证，包括但不限于：</p>
          <ul>
            <li>不保证软件的适销性</li>
            <li>不保证软件适用于特定目的</li>
            <li>不保证软件不会侵权</li>
          </ul>
          <p>在任何情况下，作者或版权持有人均不对因软件或软件的使用或其他交易而引起的任何索赔、损害或其他责任负责，无论是合同诉讼、侵权行为还是其他方面。</p>
        </div>
      </div>

      <div class="license-section">
        <div class="section-title">开源组件</div>
        <div class="section-content">
          <p>本应用基于以下开源技术开发，感谢开源社区的贡献：</p>
          <ul>
            <li>Tauri - 构建跨平台桌面应用的框架</li>
            <li>Vue 3 - 渐进式 JavaScript 框架</li>
            <li>Element Plus - Vue 3 组件库</li>
            <li>TypeScript - JavaScript 的超集</li>
            <li>Rust - 系统编程语言</li>
          </ul>
        </div>
      </div>

      <div class="license-section">
        <div class="section-title">第三方许可</div>
        <div class="section-content">
          <p>本应用可能包含第三方开源组件，这些组件可能受其各自的许可证约束。有关第三方组件的详细许可信息，请参阅各组件的官方网站或源代码仓库。</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.license-page {
  height: 100%;
  background: #18191C;
  overflow-y: auto;
}

.license-content {
  max-width: 800px;
  margin: 0 auto;
  padding: 24px;
}

.page-title {
  font-size: 20px;
  font-weight: 500;
  color: #E0E6ED;
  margin: 0 0 32px 0;
  text-align: center;
}

.license-section {
  margin-bottom: 24px;
}

.section-title {
  font-size: 15px;
  font-weight: 500;
  color: #E0E6ED;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid #2A2B30;
}

.section-content {
  color: #C8D0DC;
  font-size: 13px;
  line-height: 1.8;
}

.section-content p {
  margin-bottom: 12px;
}

.section-content ul {
  padding-left: 20px;
  margin-bottom: 12px;
}

.section-content li {
  margin-bottom: 8px;
  list-style-type: disc;
}
</style>