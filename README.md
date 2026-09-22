# LeafTidy 轻羽归档

**纯离线 · 永久免费的跨平台文件整理工具**

[![Stars](https://img.shields.io/github/stars/YangShengzhou03/LeafTidy?style=for-the-badge&color=f59e0b)](https://github.com/YangShengzhou03/LeafTidy/stargazers) [![License](https://img.shields.io/github/license/YangShengzhou03/LeafTidy?style=for-the-badge&color=blue)](https://github.com/YangShengzhou03/LeafTidy/blob/main/LICENSE) [![Release](https://img.shields.io/github/v/release/YangShengzhou03/LeafTidy?style=for-the-badge&color=success)](https://github.com/YangShengzhou03/LeafTidy/releases) [![Tauri](https://img.shields.io/badge/Tauri-2.x-FFC131?style=for-the-badge&logo=tauri&logoColor=000)](https://github.com/tauri-apps/tauri) [![Vue 3](https://img.shields.io/badge/Vue-3-4FC08D?style=for-the-badge&logo=vue.js&logoColor=white)](https://vuejs.org) ![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-eeeeee?style=for-the-badge) ![Offline](https://img.shields.io/badge/Offline-100%25-00b894?style=for-the-badge)

<a href="https://get.microsoft.com/installer/download/9pfntg4l910r?referrer=appbadge" target="_self">
  <img src="https://get.microsoft.com/images/en-us%20dark.svg" width="200"/>
</a>

> 基于 Tauri 2 构建的本地文件整理工具，所有运算在本机完成，无需联网。支持智能归档、批量重命名、内容级去重、EXIF 隐私清除、照片 GPS 写入、拍摄时间修复、系统垃圾清理，所有操作支持一键撤回。

![轻羽海报](https://gitee.com/Yangshengzhou/yang-shengzhou/raw/master/LeafTidy/assets/LeafTidy-poster-cn.png)

---

## 功能

### 智能归档

按文件类型、修改时间、拍摄时间、文件大小等多维度组合归档。支持自定义层级路径（如「年份/月份/类型」），通过拖拽标签自由组合归档层级，一键生成规整的文件夹目录结构。

- 时间来源可选：文件修改时间、创建时间、照片拍摄时间
- 支持文件类型、时间、大小等多种归档标签
- 路径构建器实时预览目标目录结构
- 归档进度实时显示，成功/失败文件分别统计
- 支持同时添加多个工作目录，可指定输出位置

### 批量重命名

内置可视化模板构建器，通过拖拽标签和分隔符自由组合命名规则，支持实时预览改名效果。

- 时间来源可选：文件修改时间、创建时间、照片拍摄时间
- 支持自定义序列编号起始值
- 内置多种命名标签（文件名、扩展名、日期、时间、序号等）
- 支持自定义分隔符（下划线、横线、空格等）
- 模板构建器实时预览，点击标签可移除
- 批量处理上千份文件，进度实时显示

### 重复文件清理

基于文件内容哈希校验（MD5），不依靠文件名，只比对文件真实内容。无论文件经过改名、复制或格式转换，只要内容一致即可精准识别。

- 支持多种检测模式可选
- 处理模式支持：直接删除、移至指定文件夹
- 扫描结果按重复组展示，显示 MD5、文件大小、重复数量
- 统计可释放磁盘空间
- 支持全选/取消全选、按组选择重复文件
- 原始文件自动标记，避免误删

### EXIF 隐私清除

照片包含大量敏感元数据：GPS 定位、拍摄设备型号、镜头参数、拍摄时间等，随意分享容易泄露隐私。支持选择性清除特定字段，在保留照片画质的同时守护个人隐私。

- 支持清除 EXIF 元数据
- 支持清除 GPS 定位信息
- 支持清除相机参数（型号、镜头、光圈、快门等）
- 支持清除 XMP 元数据
- 支持清除 Photoshop 元数据
- 可自由选择清除项，支持批量处理
- 处理结果展示成功/失败明细

### 照片 GPS 写入

为照片写入 GPS 坐标信息，支持预设位置快速填充和手动输入经纬度。内置反向地理编码功能，可根据坐标查询对应地址信息。

- 支持手动输入经纬度（精度 6 位小数）
- 内置预设位置快速填充
- 反向地理编码：输入坐标自动查询地址
- 批量写入，进度实时显示
- 处理结果展示成功/失败明细

### 拍摄时间修复

修正照片错乱的拍摄日期，支持批量统一设置时间或按文件单独调整。可基于文件名中的日期或手动指定时间进行修复，让相册按正确时间线排列。

- 日期来源可选：从文件名解析、手动指定
- 支持批量统一设置时间
- 日期时间选择器，精确到秒
- 批量处理，进度实时显示
- 处理结果展示成功/失败明细

### 系统垃圾清理

Windows、macOS 系统运行中会持续生成各类隐藏冗余文件，手动难以彻底清除。支持一键批量扫描并清理系统专属冗余文件。

- 支持清理缩略图缓存（Thumbs.db）
- 支持清理临时文件
- 支持清理 .DS_Store（macOS）
- 支持清理 desktop.ini（Windows）
- 扫描结果按类型分组展示，显示文件数量和占用空间
- 支持勾选/取消勾选，可点击文件路径快速定位

### 操作日志与撤回

完整记录每次整理、移动、重命名操作，生成本地操作日志。任何时候对整理效果不满意，都可以一键全盘撤回所有操作，文件恢复至原始状态。

- 日志列表展示：操作时间、类型、源路径、目标路径、状态
- 支持按操作类型筛选（归档、重命名、去重、清理等）
- 日志详情页展示完整操作记录
- 支持一键全量撤回任意操作
- 支持删除单条日志、清空全部日志

---

## 预览

| 智能归档 | 重复清理 |
| :---: | :---: |
| ![智能归档](https://gitee.com/Yangshengzhou/yang-shengzhou/raw/master/LeafTidy/assets/LeafTidy-smart-organize.png) | ![重复清理](https://gitee.com/Yangshengzhou/yang-shengzhou/raw/master/LeafTidy/assets/LeafTidy-file-deduplication.png) |

| EXIF 清除 | 照片导入 |
| :---: | :---: |
| ![照片处理](https://gitee.com/Yangshengzhou/yang-shengzhou/raw/master/LeafTidy/assets/LeafTidy-attribute-write.png) | ![相册归类](https://gitee.com/Yangshengzhou/yang-shengzhou/raw/master/LeafTidy/assets/LeafTidy-media-import.png) |

---

## 下载

推荐通过微软应用商店安装，自动更新且无需额外操作：

其他渠道：[GitHub Release](https://github.com/YangShengzhou03/LeafTidy/releases)（Windows / macOS / Linux）、[联想应用商店](https://lestore.lenovo.com/detail)、[蓝奏云](https://cca4666.lanzoul.com/b037bh2o1c)（提取密码：c9d7）、[123云盘](https://www.123865.com/s/wgLiVv-hj7v3)。

### 使用流程

```
添加文件夹 → 选择功能 → 配置规则 → 预览确认 → 一键执行 → 支持撤回
```

打开软件首页，添加待整理的文件夹（支持同时添加多个目录），可指定输出位置。在功能卡片中进入对应功能（归档、去重、改名、EXIF 等），根据需求配置参数。执行前预览操作结果，确认无误后点击开始。操作完成后可在日志页面查看详情，支持一键撤回恢复。

软件首页提供首次使用引导教程，逐步介绍工作目录、输出位置、功能入口等区域，零基础用户也能快速上手。

---

## 为什么选择 LeafTidy

市面上大量传统整理工具采用持续订阅收费模式，却存在诸多短板：底层技术迭代缓慢，大批量处理时卡顿明显；界面功能堆砌，操作路径繁琐；识别逻辑单一，仅靠文件名判断重复；依赖联网运行，隐私保护不足。

LeafTidy 的差异化优势在于：所有核心功能永久免费开放，无试用时限、无执行次数限制、无广告无弹窗诱导；纯离线运行，无需联网激活、无需登录账号，所有运算在本机完成，数据不出本地；基于 Tauri 2 构建，安装包体积仅数 MB，老旧电脑也能流畅运行；哈希校验文件真实内容，不受文件名、格式转换影响，清理更彻底；EXIF/GPS/相机参数等敏感信息一键清除，填补传统工具功能空白；完整操作日志配合一键全量撤回，误操作可完全恢复；兼容 Windows、macOS、Linux 主流操作系统。

---

## 技术栈

前端采用 [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Element Plus](https://element-plus.org/)，利用组合式 API 和响应式 UI 组件库构建流畅界面。桌面框架使用 [Tauri 2](https://tauri.app/)（Rust），利用系统原生 WebView，安装包体积极小，性能远超 Electron。构建工具为 [Vite](https://vitejs.dev/)，提供极速热重载和高效构建。数据可视化使用 [ECharts](https://echarts.apache.org/)，展示文件类型分布、存储空间统计等图表。内置中/英/繁中三语国际化支持。

### 源码构建

前置条件：[Node.js](https://nodejs.org/) >= 18、[Rust](https://www.rust-lang.org/tools/install) 最新稳定版。

```bash
git clone https://github.com/YangShengzhou03/LeafTidy.git
cd LeafTidy
npm install
npm run tauri dev    # 开发模式
npm run tauri build  # 构建发布包
```

构建产物位于 `src-tauri/target/release/`。

---

## 项目结构

```
leaf-tidy/
├── src/                      # 前端源码 (Vue 3 + TypeScript)
│   ├── components/           # 公共组件（左侧栏、右侧栏等）
│   ├── composables/          # 组合式函数（文件操作、布局、日志等）
│   ├── i18n/                # 国际化（中/英/繁中三语）
│   ├── views/               # 功能页面
│   │   ├── HomePage.vue           # 首页（工作目录、功能入口、新手引导）
│   │   ├── FileOrganizePage.vue   # 智能归档
│   │   ├── BatchRenamePage.vue    # 批量重命名
│   │   ├── DuplicateCleanPage.vue # 重复文件清理
│   │   ├── ExifCleanPage.vue      # EXIF 隐私清除
│   │   ├── WriteGpsPage.vue       # 照片 GPS 写入
│   │   ├── FixDatePage.vue        # 拍摄时间修复
│   │   ├── CleanupPage.vue        # 系统垃圾清理
│   │   ├── LogViewPage.vue        # 操作日志列表
│   │   ├── LogDetailPage.vue     # 操作日志详情
│   │   ├── SettingsPage.vue       # 设置
│   │   ├── AboutPage.vue          # 关于
│   │   └── ...
│   ├── App.vue              # 应用根组件
│   └── main.js              # 入口文件
├── src-tauri/               # Tauri 后端 (Rust)
│   ├── src/
│   │   ├── main.rs          # 程序入口
│   │   ├── commands.rs      # Tauri 命令注册
│   │   ├── file_ops.rs      # 文件操作（移动、复制、删除等）
│   │   ├── metadata.rs      # 文件元数据处理（EXIF 读写）
│   │   ├── geocode.rs       # 地理编码（坐标反查地址）
│   │   ├── log.rs           # 操作日志系统
│   │   ├── models.rs        # 数据模型定义
│   │   └── cancel.rs        # 操作取消支持
│   ├── Cargo.toml           # Rust 依赖配置
│   └── tauri.conf.json      # Tauri 应用配置
├── package.json             # Node 依赖与脚本
├── tsconfig.json           # TypeScript 配置
└── README.md
```

---

## 常见问题

**Q: 免费版功能是否弱于付费软件？**

不会。LeafTidy 依托全新自研技术架构搭建，在智能识别能力、运行速度、重复文件识别精度、隐私保护机制、操作便捷性上均优于大量传统付费整理工具。传统工具依靠老旧架构持续收费，本工具选择全新体验永久免费开放，给用户多一个优质选择。

**Q: 离线运行是否功能缺失？**

不会。所有核心功能完全本地化实现，无需联网。

**Q: 误操作后如何恢复？**

进入「日志」页面，支持一键全量撤回任意操作，文件恢复至原始状态。

**Q: 支持哪些图片格式？**

EXIF 相关功能支持 JPEG、TIFF、PNG、HEIC/HEIF 等常见格式。

**Q: 老旧电脑能否流畅运行？**

可以。基于 Tauri 构建，安装包体积仅数 MB，硬件资源占用极低。

**Q: 支持哪些操作系统？**

兼容 Windows、macOS、Linux 主流操作系统。

**Q: 软件是否支持多语言？**

支持简体中文、英语、繁体中文三种语言，可在设置中切换。

---

## 许可

本项目基于 [MIT](https://github.com/YangShengzhou03/LeafTidy/blob/main/LICENSE) 协议开源。

---

**让每一位用户，零成本、零门槛拥有专业级文件整理能力。**
