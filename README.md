# LeafTidy · 轻羽归档

[![License](https://img.shields.io/github/license/YangShengzhou03/LeafTidy?style=flat)](https://github.com/YangShengzhou03/LeafTidy/blob/master/LICENSE) [![GitHub Stars](https://img.shields.io/github/stars/YangShengzhou03/LeafTidy?style=flat)](https://github.com/YangShengzhou03/LeafTidy/stargazers) [![GitHub Issues](https://img.shields.io/github/issues/YangShengzhou03/LeafTidy?style=flat)](https://github.com/YangShengzhou03/LeafTidy/issues) [![GitHub Last Commit](https://img.shields.io/github/last-commit/YangShengzhou03/LeafTidy?style=flat)](https://github.com/YangShengzhou03/LeafTidy/commits/main) [![Tauri Version](https://img.shields.io/badge/Tauri-2.x-blue?style=flat)](https://tauri.app/) [![Vue Version](https://img.shields.io/badge/Vue-3.x-green?style=flat)](https://vuejs.org/) [![Rust Version](https://img.shields.io/badge/Rust-1.77+-orange?style=flat)](https://www.rust-lang.org/) [![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat)](https://github.com/YangShengzhou03/LeafTidy/releases)

---

## 简介

**LeafTidy** 是一款专注于文件整理的跨平台桌面应用。它采用纯离线设计，无需联网，你的文件永远不会离开本地设备。

> 安静、高效、安全 —— 让文件整理变得简单

---

## 功能特性

### 文件整理

- **智能分类**：按文件类型、日期、大小自动整理到统一目录
- **保留原结构**：支持保持原有目录层级的搬运模式
- **混合规则归档**：年份/月份/省份/AI标签多层组合归档

### 批量重命名

- **模板支持**：支持 `{日期}_{原名}`、`{序号}_{类别}` 等灵活模板
- **自定义序号**：支持2位到6位序号格式

### 重复文件清理

- **MD5 校验**：基于内容的精确重复检测
- **大小过滤**：可选只删除完全相同或大小相同的文件
- **预览确认**：删除前提供完整列表供确认

### 照片处理

- **修复拍摄时间**：用文件修改时间或自定义时间修正错误的 EXIF 时间
- **照片脱敏**：移除 GPS 位置、相机型号等隐私信息
- **GPS 位置分类**：基于离线地名库，按省/市/区县自动分类
- **AI 智能分类**：本地 AI 模型识别照片内容，自动归类人像、风景、宠物、美食等

### 其他工具

- **文件筛选**：按类型、大小、时间范围精准过滤
- **附属文件清理**：清理 Thumbs.db、.DS_Store、desktop.ini 等系统垃圾文件

### 日志与回滚

- **完整日志**：每次操作自动记录详细日志
- **一键回滚**：支持移动和重命名操作的完整回滚

---

## 截图展示

> 即将上线...

---

## 安装

### 预构建版本

访问 [Releases](https://github.com/YangShengzhou03/LeafTidy/releases) 页面下载对应平台的安装包：

- **Windows**: `.exe` 安装程序
- **macOS**: `.dmg` 磁盘镜像
- **Linux**: `.deb` / `.rpm` / AppImage

### 从源码构建

需要环境：
- Node.js ≥ 18
- Rust ≥ 1.77

```bash
# 克隆仓库
git clone https://github.com/YangShengzhou03/LeafTidy.git
cd LeafTidy

# 安装前端依赖
npm install

# 开发模式运行
npm run dev

# 生产打包
npm run build:tauri
```

---

## 使用指南

1. **选择功能**：从左侧导航栏选择需要的功能
2. **添加文件/文件夹**：点击添加按钮或拖拽文件到应用
3. **配置参数**：根据需要设置整理规则、重命名模板等
4. **预览结果**：查看操作预览列表，确认无误
5. **执行操作**：点击执行按钮，完成整理
6. **查看日志**：如需回滚，从日志页面找到对应操作

---

## 技术栈

### 前端

| 技术 | 版本 | 说明 |
|------|------|------|
| Vue | 3.x | 渐进式 JavaScript 框架 |
| TypeScript | 4.9+ | 类型安全的 JavaScript |
| Element Plus | 2.x | Vue 3 组件库 |
| ECharts | 6.x | 数据可视化图表库 |
| Tauri API | 2.x | Tauri 前端 API |

### 后端

| 技术 | 版本 | 说明 |
|------|------|------|
| Rust | 1.77+ | 系统级编程语言 |
| Tauri | 2.x | 桌面应用框架 |
| kamadak-exif | 0.5 | EXIF 信息解析 |
| walkdir | 2.x | 递归目录扫描 |
| chrono | 0.4 | 时间处理 |
| md-5 | 0.10 | MD5 哈希计算 |
| trash | 3.x | 安全删除到回收站 |

---

## 项目结构

```
LeafTidy/
├── src/                    # 前端代码
│   ├── components/         # UI 组件
│   │   ├── LeftSidebar.vue    # 左侧导航栏
│   │   └── RightSidebar.vue   # 右侧详情栏
│   ├── composables/        # 组合式函数
│   │   ├── useFileOps.ts      # 文件操作逻辑
│   │   ├── useLayout.ts       # 布局状态管理
│   │   └── useLog.ts          # 日志管理
│   ├── types/              # TypeScript 类型定义
│   ├── views/              # 功能页面
│   │   ├── HomePage.vue       # 首页
│   │   ├── FileOrganizePage.vue # 文件整理
│   │   ├── BatchRenamePage.vue # 批量重命名
│   │   ├── DuplicateCleanPage.vue # 重复文件清理
│   │   ├── FixDatePage.vue    # 修复拍摄时间
│   │   ├── ExifCleanPage.vue  # 照片脱敏
│   │   ├── WriteGpsPage.vue   # GPS 位置分类
│   │   ├── AiClassifyPage.vue # AI 智能分类
│   │   ├── CleanupPage.vue    # 附属文件清理
│   │   ├── LogViewPage.vue    # 日志列表
│   │   ├── LogDetailPage.vue  # 日志详情
│   │   └── SettingsPage.vue   # 设置页面
│   ├── App.vue             # 主布局组件
│   ├── main.js             # 入口文件
│   └── env.d.ts            # 环境声明
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs            # 应用入口
│   │   ├── lib.rs             # 库入口
│   │   ├── commands.rs        # Tauri 命令定义
│   │   ├── file_ops.rs        # 文件读写移动操作
│   │   ├── metadata.rs        # EXIF/GPS 处理
│   │   ├── geocode.rs         # GPS 地理编码
│   │   ├── models.rs          # 数据模型
│   │   └── log.rs             # 日志和回滚逻辑
│   ├── Cargo.toml             # Rust 依赖配置
│   └── tauri.conf.json        # Tauri 配置
├── package.json            # 前端依赖配置
├── tsconfig.json           # TypeScript 配置
├── vue.config.js           # Vue CLI 配置
└── README.md               # 项目说明文档
```

---

## 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. 提交更改：`git commit -m 'Add your feature'`
4. 推送到分支：`git push origin feature/your-feature`
5. 创建 Pull Request

### 开发规范

- 遵循 Vue 3 Composition API 风格
- TypeScript 类型定义完整
- ESLint 检查通过
- 代码注释清晰

---

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

## 致谢

- [Tauri](https://tauri.app/) - 优秀的桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 组件库
- [kamadak-exif](https://crates.io/crates/kamadak-exif) - EXIF 解析库

---

**LeafTidy** — 让文件整理变得简单

[GitHub](https://github.com/YangShengzhou03/LeafTidy) · [Issues](https://github.com/YangShengzhou03/LeafTidy/issues) · [Releases](https://github.com/YangShengzhou03/LeafTidy/releases)