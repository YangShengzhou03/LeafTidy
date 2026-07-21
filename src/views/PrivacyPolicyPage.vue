<template>
  <div class="privacy-page">
    <div class="privacy-content">
      <h1 class="page-title">隐私协议</h1>

      <div class="policy-section">
        <div class="section-title">引言</div>
        <div class="section-content">
          <p>欢迎您使用轻羽归档(LeafTidy)应用程序。我们非常重视您的隐私保护和个人信息保护。本隐私协议将向您说明我们如何收集、使用、存储和保护您的信息，以及您享有的相关权利。</p>
          <p>在使用本应用前，请您仔细阅读并充分理解本协议。如果您不同意本协议的任何内容，请您立即停止使用本应用。</p>
        </div>
      </div>

      <div class="policy-section">
        <div class="section-title">信息收集</div>
        <div class="section-content">
          <p>轻羽归档是一款本地文件管理工具，我们的设计理念是最大限度保护您的隐私：</p>
          <ul>
            <li>本应用主要在您的设备本地运行，处理的照片和文件信息均存储在您的本地设备上</li>
            <li>我们不会自动收集您的个人信息、照片内容或文件信息</li>
            <li>应用不会在后台自动上传任何数据到远程服务器</li>
          </ul>
        </div>
      </div>

      <div class="policy-section">
        <div class="section-title">本地数据处理</div>
        <div class="section-content">
          <p>本应用处理的所有数据均在您的本地设备上完成：</p>
          <ul>
            <li>照片整理、重命名、清理等操作均在本地完成</li>
            <li>EXIF信息读取和修改仅在本地进行</li>
            <li>AI分类功能需要在本地模型支持下运行，相关数据不会上传到云端</li>
            <li>日志文件仅保存在您的本地应用数据目录中</li>
          </ul>
        </div>
      </div>

      <div class="policy-section">
        <div class="section-title">数据安全</div>
        <div class="section-content">
          <p>我们采取多种措施保护您的数据安全：</p>
          <ul>
            <li>所有文件操作均需您的明确授权和确认</li>
            <li>应用运行在沙盒环境中，仅能访问您明确授权的目录</li>
            <li>敏感操作会生成详细的日志记录，方便您追溯和审计</li>
          </ul>
        </div>
      </div>

      <div class="policy-section">
        <div class="section-title">您的权利</div>
        <div class="section-content">
          <p>您对自己的数据享有完整的控制权：</p>
          <ul>
            <li>您可以随时查看、修改或删除应用处理的文件</li>
            <li>您可以查看和清除应用生成的操作日志</li>
            <li>您可以随时卸载应用，卸载后应用及相关数据将从您的设备中移除</li>
          </ul>
        </div>
      </div>

      <div class="policy-section">
        <div class="section-title">协议更新</div>
        <div class="section-content">
          <p>我们可能会适时修订本隐私协议的内容。修订后的协议将在应用内公布，您可以在设置页面查看最新版本。如果您继续使用本应用，即表示您同意接受修订后的协议。</p>
        </div>
      </div>

      <div class="policy-section">
        <div class="section-title">联系我们</div>
        <div class="section-content">
          <p>如果您对本隐私协议有任何疑问或建议，请通过以下方式联系我们：</p>
          <ul>
            <li>在 GitHub 上提交 Issue</li>
            <li>通过应用内反馈功能联系我们</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.privacy-page {
  height: 100%;
  background: #18191C;
  overflow-y: auto;
}

.privacy-content {
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

.policy-section {
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