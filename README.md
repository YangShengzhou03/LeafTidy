# LeafMaster（轻羽大师版）- 专业级微信自动化系统

[![Gitee](https://img.shields.io/badge/Gitee-LeafMaster-blue)](https://gitee.com/Yangshengzhou/LeafAuto)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)

> **技术栈升级**: ElementPlus + Rust (Tauri) - 更极致的内存和速度效率

## 项目简介

LeafMaster（轻羽大师版）是轻羽信息自动系统的专业版本，采用全新的技术栈重构，面向专业用户，追求极致的性能和更强大的功能。

### 与普通版对比

| 特性 | 普通版 (LeafAuto) | 大师版 (LeafMaster) |
|------|-------------------|---------------------|
| 技术栈 | PyQt6 + Python | ElementPlus + Rust (Tauri) |
| 内存占用 | 较高 (~150MB) | 极低 (~20MB) |
| 启动速度 | 中等 | 极快 |
| 界面技术 | Win32 控件 | 现代化 Web 技术 |
| 扩展性 | 有限 | 高度可扩展 |
| 技术迭代 | 困难 | 容易（ElementPlus 生态） |

## 功能特性

### 核心功能

- **定时发送**: 支持精确到秒的定时消息和文件发送
- **间隔循环**: 按间隔时间循环执行任务
- **文件发送**: 支持发送各种格式的文件（PDF、Word、Excel、图片等）
- **消息管理**: 发送文本消息到指定联系人或群组
- **智能拆句**: 自动拆分长消息，避免消息过长
- **Excel 导入导出**: 批量管理任务配置

### 高级功能

- **多任务并发**: 同时执行多个任务（专业版）
- **任务队列管理**: 智能调度任务执行顺序
- **失败重试机制**: 自动重试失败的任务
- **实时日志监控**: 查看任务执行详情
- **网络时间同步**: 确保定时任务精准执行
- **防休眠机制**: 保持系统唤醒状态

### 即将推出

- **智能回复**: 基于关键词的自动回复功能
- **消息模板**: 预设消息模板，快速发送
- **群发助手**: 批量发送消息到多个联系人
- **数据统计**: 任务执行统计和分析报告

## 快速开始

### 系统要求

- **操作系统**: Windows 10 64位或更高版本
- **微信版本**: 3.9.0 及以上版本
- **内存**: 建议 8GB RAM（推荐 16GB）
- **存储空间**: 100MB 可用空间

### 安装使用

1. **下载安装包**
   - 从 [Gitee 发行版](https://gitee.com/Yangshengzhou/LeafAuto/releases) 下载最新版本

2. **安装程序**
   - 双击安装包，按照向导完成安装

3. **登录微信**
   - 确保微信已登录并保持运行状态
   - 建议将微信窗口保持在桌面上

4. **创建任务**
   - 打开 LeafMaster，进入"自动信息"
   - 创建新任务，设置发送内容、接收对象和时间
   - 启用任务，享受自动化

### 使用技巧

- 建议给联系人设置 5 个纯汉字以内的备注，避免特殊字符
- 设置任务时间时，建议提前 1-2 分钟，避免网络延迟
- 定期备份任务配置到 Excel 文件
- 定期清理已完成的任务，保持列表整洁

## 开发指南

### 技术栈

- **前端**: Vue 3 + Element Plus + TypeScript
- **后端**: Rust + Tauri
- **状态管理**: Pinia
- **构建工具**: Vite

### 本地开发

```bash
# 克隆仓库
git clone https://gitee.com/Yangshengzhou/LeafAuto.git
cd LeafAuto
git checkout leaf-master

# 安装依赖
npm install

# 开发模式运行
npm run start

# 构建生产版本
npm run build
```

### 项目结构

```
leaf-master/
├── src/                  # Vue 前端源码
│   ├── pages/           # 页面组件
│   ├── router/          # 路由配置
│   └── stores/          # 状态管理
├── src-tauri/           # Rust 后端源码
│   ├── src/             # Rust 源码
│   └── tauri.conf.json  # Tauri 配置
└── package.json         # Node.js 配置
```

## 版本说明

### 轻羽大师版（LeafMaster）

**定价方案：试用 + 买断制**
- **试用版**：免费试用 7 天，功能无限制
- **正式版**：¥399 永久授权，一次购买终身使用

**包含功能：**
- 无限任务数和每日任务数
- 定时发送、间隔循环、文件发送
- Excel 批量导入导出任务
- 智能拆句、失败重试、多任务并发
- 掉线检测、邮件提醒、防休眠等全部高级功能

### 增值服务

| 服务项目 | 价格 |
|---------|------|
| 售后咨询服务 | ¥29.9 起 |
| 功能维护服务 | ¥49.9 起 |
| 使用教程培训 | ¥69.9 起 |
| 远程调试服务 | ¥99 起 |

**购买方式**：联系技术支持获取激活卡密
**支付方式**：微信、支付宝、银行转账

## 技术支持

- **邮箱**: yangsz03@foxmail.com
- **工作时间**: 周一至周五 9:00-18:00（法定节假日除外）
- **Gitee**: [https://gitee.com/Yangshengzhou/LeafAuto](https://gitee.com/Yangshengzhou/LeafAuto)

## 致谢

感谢所有使用和支持 LeafMaster 的用户。感谢开源社区和开源项目的支持。

## 版权声明

Copyright © 2025-2026 Yangshengzhou. All rights reserved.

---

**LeafMaster（轻羽大师版）- 专业级微信自动化，让效率更极致！**