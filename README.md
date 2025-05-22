# Personal Collector - Tauri + Vue 3 + TypeScript 桌面应用模板

> 基于 [Tauri](https://tauri.app/)、[Vue 3](https://vuejs.org/guide/introduction.html) 和 [TypeScript](https://www.typescriptlang.org/) 构建的跨平台桌面应用模板。

---

## 🚀 项目简介

该项目是一个轻量级桌面应用模板，结合 Vue 3 Composition API、TypeScript 和 Tauri 实现本地化能力，适合用于构建现代跨平台桌面应用。

### 主要功能模块：
- ✅ 待办事项管理（增删改查）
- ☀️ 城市天气预报
- 🎵 本地音乐播放器

---

## 🛠 技术栈

| 技术 | 描述 |
|------|------|
| **Vue 3** | 使用 `<script setup>` 语法 |
| **TypeScript** | 类型安全的 JavaScript 超集 |
| **Vite** | 快速冷启动的下一代前端构建工具 |
| **Tauri** | 轻量级替代 Electron 的桌面应用框架 |
| **Rust** | 后端逻辑与数据库交互（SQLite） |
| **Vuetify** | Material Design 风格组件库 |

---

## 💡 推荐 IDE 设置

- [VS Code](https://code.visualstudio.com/)
  - [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### 启用 Volar Take Over 模式

1. 打开命令面板：`Extensions: Show Built-in Extensions`
2. 禁用 `TypeScript and JavaScript Language Features`
3. 重新加载 VS Code：`Developer: Reload Window`

了解更多：[Volar Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471)

---

## 🧪 快速开始

```bash
# 安装依赖（支持 pnpm/npm/yarn）
pnpm install
# npm install
# yarn install

# 启动开发模式
pnpm tauri dev
```

---

## 🧱 构建与发布

```bash
# 构建生产版本
pnpm tauri build
```

这将生成适用于 Windows/macOS/Linux 的原生应用包。

---

## 📷 效果图

![项目截图](pic.png)

> 图中展示了应用的主要功能界面：待办事项、天气卡片与本地音乐播放器。

---

## 📁 目录结构

```bash
personal-collector/
├── src/                # Vue 前端源码
├── src-tauri/          # Tauri Rust 后端逻辑
├── pic.png             # 项目截图
└── README.md           # 当前文档
```

---

## 📜 License

本项目采用 MIT License。详见 [LICENSE](./LICENSE)。

---

如果你需要我帮你补充 `LICENSE` 文件或其他配置文件，请告诉我！