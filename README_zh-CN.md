<h1 align="center">aichat</h1>
<h4 align="center">
一款基于终端的AI聊天应用程序,纯粹为了娱乐而创建. 基于 <a href="https://github.com/ratatui/ratatui">ratatui</a>
</h4>
<p align="center">
  <a href="https://https://github.com/0xhappyboy/aichat/LICENSE"><img src="https://img.shields.io/badge/License-GPL3.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

# 演示

<img src="./assets/demo.gif" alt="ai chat demo" width="100%">

# 环境变量

```
DEEPSEEK_API_KEY=<your deepseek api key>
ALIYUN_API_KEY=<your aliyun api key>
```

# 快捷键

## 通用

- Enter - 开始聊天 / 发送消息
- F1 - 显示 / 隐藏帮助
- C - 切换到中文界面
- E - 切换到英文界面
- 1-4 - 切换主题 (1:蓝色, 2:绿色, 3:橙色, 4:霓虹)
- Q - 退出应用

## 欢迎界面

- Enter - 开始聊天
- C - 切换到中文界面
- E - 切换到英文界面
- 1-4 - 切换主题
- F1 - 显示帮助
- Q - 退出应用

## 聊天界面 (正常模式)

- ← → - 切换 AI 模型
- ↑ ↓ - 上下滚动聊天历史
- PageUp - 向上滚动 10 行
- PageDown - 向下滚动 10 行
- Home - 跳转到顶部
- End - 跳转到底部

## 输入控制

- i - 进入编辑模式
- Esc - 退出编辑模式 (清除输入)
- Enter - 发送消息并返回正常模式

## 编辑模式

- Enter - 发送消息并退出编辑模式
- Esc - 退出编辑模式 (不发送消息)
- 字母/数字 - 输入文本
- Backspace - 删除最后一个字符
- Delete - 清空输入框

# 主题

<table>
  <tr>
    <td align="left">
    <h4>蓝色</h4>
    </td>
    <td align="left">
    <h4>绿色</h4>
    </td>
    <td align="left">
    <h4>橘黄色</h4>
    </td>
    <td align="left">
    <h4>霓虹</h4>
    </td>
  </tr>
  <tr>
    <td align="center"><img src="./assets/theme/blue.png" width="400"></td>
    <td align="center"><img src="./assets/theme/green.png" width="400"></td>
    <td align="center"><img src="./assets/theme/orange.png" width="400"></td>
    <td align="center"><img src="./assets/theme/neon.png" width="400"></td>
  </tr>
</table>
