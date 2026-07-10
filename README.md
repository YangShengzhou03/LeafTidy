# LeafTidy · 轻羽归档

<div align="center">

**安静、高效、安全 —— 让文件整理变得简单**

[![GitHub stars](https://img.shields.io/github/stars/YangShengzhou03/LeafTidy?style=for-the-badge&logo=github)](https://github.com/YangShengzhou03/LeafTidy/stargazers) [![GitHub forks](https://img.shields.io/github/forks/YangShengzhou03/LeafTidy?style=for-the-badge&logo=github)](https://github.com/YangShengzhou03/LeafTidy/network/members) [![GitHub issues](https://img.shields.io/github/issues/YangShengzhou03/LeafTidy?style=for-the-badge&logo=github)](https://github.com/YangShengzhou03/LeafTidy/issues) [![GitHub license](https://img.shields.io/github/license/YangShengzhou03/LeafTidy?style=for-the-badge)](https://github.com/YangShengzhou03/LeafTidy/blob/main/LICENSE)

</div>

---

## 简介

数字时代，我们每个人都在不断积累文件。照片、文档、视频，这些数字资产散落在硬盘的各个角落，整理它们成了一件让人头疼的事。你有没有过这样的经历？手机里存了几千张照片，想找去年夏天去海边的那张，翻了半小时也没找到；电脑里下载的文件堆得像小山，不知道哪些是有用的；整理了半天的文件，一不小心删错了又找不回来。

LeafTidy 就是为了解决这个问题而生的。它就像你的私人文件管家，安静、高效、安全地帮你打理好每一个文件。这款跨平台桌面应用采用纯离线设计，所有操作都在你的本地设备上完成，你的文件永远不会离开你的电脑。这意味着你不用担心隐私泄露，也不需要担心网络问题影响使用。

## 功能特点

### 文件整理

文件整理功能是一切的基础。你是否曾经把照片、文档、视频一股脑塞进一个文件夹里，然后就再也找不到想要的文件了？LeafTidy 可以按文件类型、日期、大小等多种规则自动整理到统一目录。你可以保持原有目录层级，也可以根据年份、月份、省份等多层规则进行组合归档，让每一个文件都能找到属于它的位置。

### 批量重命名

文件名混乱是文件整理的另一大痛点。那些"IMG_20230520_143021.jpg"、"DSC001.jpg"、"照片1.jpg"之类的名字让你根本不知道文件内容是什么。LeafTidy 的批量重命名功能支持灵活的模板，比如 `{日期}_{原名}`、`{序号}_{类别}` 等，还支持自定义序号格式，一次操作就能给成百上千个文件起上清晰统一的名字。

### 重复清理

随着时间的推移，我们总会在不同地方保存相同的文件——可能是备份了好几次，也可能是下载了重复的资源。这些重复文件占用了大量存储空间，却毫无价值。LeafTidy 使用 MD5 校验技术，基于文件内容进行精确的重复检测。你可以选择只删除完全相同的文件，也可以按大小过滤，删除前还会提供完整列表供你确认，让你安心清理。

### 照片处理工具

照片是我们最珍贵的数字资产，但它们常常存在各种问题。LeafTidy 提供了专业的 EXIF 信息清理功能，可以移除 GPS 位置、相机型号等隐私信息，保护你的个人数据安全。基于离线地名库，还支持按省/市/区县自动分类照片。

> **开发中**：AI 智能分类、拍摄时间修复、GPS 写入等功能正在开发中，敬请期待。

### 系统清理

除了核心功能，LeafTidy 还提供了一些实用的小工具。它可以清理 Thumbs.db、.DS_Store、desktop.ini 等系统垃圾文件，让你的文件夹更干净。

### 操作日志

操作失误是难免的——不小心删错了文件、重命名出了问题……这些都会让人非常焦虑。LeafTidy 会自动记录每一次操作的详细日志，支持按类型筛选和详情查看。你可以随时找到并撤销之前的操作，让文件恢复到原来的状态。

## 技术栈

LeafTidy 采用现代化的技术栈，兼顾性能和安全性：

- **前端框架**: Vue 3 + TypeScript
- **UI 组件库**: Element Plus
- **图标库**: Element Plus Icons
- **图表库**: ECharts
- **后端框架**: Tauri 2 + Rust
- **构建工具**: Vue CLI + Webpack
- **代码规范**: ESLint

## 快速开始

### 下载安装

访问 [Releases](https://github.com/YangShengzhou03/LeafTidy/releases) 页面下载对应平台的安装包：

- **Windows**: 下载 `.exe` 安装程序

> **macOS / Linux 用户**：目前预编译安装包仅支持 Windows。其他平台需要从源码编译，详见下方开发指南。

### 使用说明

使用 LeafTidy 非常简单，只需几个步骤：

1. 从左侧导航栏选择你需要的功能，比如文件整理、批量重命名等；
2. 点击添加按钮或直接拖拽文件到应用中；
3. 根据需要设置整理规则、重命名模板等；
4. 查看操作预览列表，确认无误后再执行；
5. 点击执行按钮，LeafTidy 会自动完成整理工作；
6. 如需回滚，从日志页面找到对应操作即可。

## 开发指南

如果你是技术爱好者，想自己编译项目，需要准备以下环境：

- **Node.js**: ≥ 18
- **Rust**: ≥ 1.77

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/YangShengzhou03/LeafTidy.git
cd LeafTidy

# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

### 构建生产版本

```bash
# 构建前端
npm run build

# 构建 Tauri 应用
npm run build:tauri
```

## 项目结构

```
LeafTidy/
├── src/                      # 前端源码
│   ├── components/           # 通用组件
│   │   ├── LeftSidebar.vue   # 左侧导航栏
│   │   └── RightSidebar.vue  # 右侧状态栏
│   ├── composables/          # 组合式函数
│   │   ├── useFileOps.ts     # 文件操作逻辑
│   │   ├── useLayout.ts      # 布局管理
│   │   └── useLog.ts         # 日志记录
│   ├── views/                # 页面组件
│   │   ├── HomePage.vue           # 首页
│   │   ├── FileOrganizePage.vue   # 文件整理
│   │   ├── BatchRenamePage.vue    # 批量重命名
│   │   ├── DuplicateCleanPage.vue # 重复清理
│   │   ├── CleanupPage.vue        # 附属清理
│   │   ├── FixDatePage.vue        # 日期修复
│   │   ├── ExifCleanPage.vue      # EXIF清理
│   │   ├── WriteGpsPage.vue       # GPS写入
│   │   ├── AiClassifyPage.vue     # AI分类
│   │   ├── LogViewPage.vue        # 日志列表
│   │   ├── LogDetailPage.vue      # 日志详情
│   │   └── SettingsPage.vue       # 设置页面
│   ├── types/                # TypeScript 类型定义
│   ├── App.vue               # 根组件
│   └── main.js               # 入口文件
├── src-tauri/                # Tauri 后端源码
│   ├── src/
│   │   ├── commands.rs       # Tauri 命令定义
│   │   ├── file_ops.rs       # 文件操作实现
│   │   ├── log.rs            # 日志管理
│   │   ├── metadata.rs       # 元数据处理
│   │   ├── geocode.rs        # 地理编码
│   │   └── models.rs         # 数据模型
│   └── Cargo.toml            # Rust 依赖配置
├── public/                   # 静态资源
├── vue.config.js             # Vue CLI 配置
├── package.json              # 前端依赖配置
└── README.md                 # 项目说明文档
```

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](https://github.com/YangShengzhou03/LeafTidy/blob/main/LICENSE) 文件。

## 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库；
2. 创建特性分支：`git checkout -b feature/your-feature`；
3. 提交更改：`git commit -m 'Add your feature'`；
4. 推送到分支：`git push origin feature/your-feature`；
5. 创建 Pull Request。

---

**LeafTidy** — 让文件整理变得简单

[GitHub](https://github.com/YangShengzhou03/LeafTidy) · [Issues](https://github.com/YangShengzhou03/LeafTidy/issues) · [Releases](https://github.com/YangShengzhou03/LeafTidy/releases)