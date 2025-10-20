# 扫雷游戏 (Minesweeper)

一个使用 Rust 和 eframe/egui 开发的经典扫雷游戏，支持中文界面。

## 功能特性

- ? 经典扫雷游戏玩法
- ? 现代化的图形界面
- ?? 完整的中文支持
- ? 智能雷区生成（首次点击安全）
- ? 双重胜利条件判定
- ? 彩色数字显示
- ?? 鼠标左右键操作

## 游戏规则

- **左键点击**：翻开格子
- **右键点击**：标记/取消标记地雷
- **目标**：找出所有地雷而不踩到它们
- **胜利条件**：
  - 翻开所有非地雷格子，或
  - 正确标记所有地雷

## 技术栈

- **语言**：Rust
- **GUI框架**：eframe + egui
- **随机数生成**：rand
- **字体**：Noto Sans SC（支持中文）

## 系统要求

- Rust 1.70+ 
- Windows/macOS/Linux

## 安装与运行

### 1. 克隆项目

```bash
git clone <repository-url>
cd Minesweeper
```

### 2. 安装 Rust

如果您还没有安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 安装。

### 3. 运行游戏

```bash
cargo run
```

### 4. 构建发布版本

```bash
cargo build --release
```

发布版本将生成在 `target/release/` 目录下。

## 项目结构

```
src/
├── main.rs      # 程序入口
├── app.rs       # 主应用程序逻辑和UI
├── board.rs     # 游戏板逻辑
└── fonts.rs     # 字体设置

assets/
└── fonts/
    └── NotoSansSC-VariableFont_wght.ttf  # 中文字体文件
```

## 游戏配置

当前游戏配置（可在 `src/board.rs` 中修改）：

- **游戏板大小**：9×9
- **地雷数量**：10个
- **格子大小**：40×40像素

## 开发说明

### 核心模块

- **`Board`**：游戏板结构，包含所有游戏逻辑
- **`Cell`**：单个格子状态（地雷、邻居地雷数、显示状态）
- **`MyApp`**：主应用程序，处理用户界面和事件

### 关键算法

1. **地雷生成**：使用随机数生成器，确保首次点击安全
2. **邻居计数**：计算每个格子周围的地雷数量
3. **自动展开**：点击空白格子时自动展开相邻区域
4. **胜利判定**：支持两种胜利条件

## 自定义配置

您可以通过修改 `src/board.rs` 中的常量来调整游戏参数：

```rust
pub const WIDTH: usize = 9;    // 游戏板宽度
pub const HEIGHT: usize = 9;   // 游戏板高度  
pub const MINES: usize = 10;   // 地雷数量
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 致谢

- 感谢 [egui](https://github.com/emilk/egui) 提供优秀的 GUI 框架
- 感谢 Google 提供的 [Noto Sans SC](https://fonts.google.com/noto/specimen/Noto+Sans+SC) 字体
