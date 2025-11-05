\# Path-Demo



一个用 Rust 编写的命令行工具，用于记忆和快速跳转到常用路径。



\## 功能特性



\- 🚀 快速添加路径别名

\- 📁 一键跳转到目标目录

\- 📋 查看所有保存的路径

\- 🗑️ 删除不再需要的路径别名

\- 💾 配置自动持久化



\## 安装



\### 前提条件

\- 安装 \[Rust](https://www.rust-lang.org/)



\### 从源码安装

```bash

\# 克隆项目

git clone https://github.com/wf5440/path-demo.git

cd path-demo



\# 编译安装

cargo install --path .



使用方法

bash

\# 添加路径别名

m add project "D:\\MyProjects"



\# 查看所有路径

m list



\# 快速跳转

to project



\# 删除别名

m remove project



配置

安装后，需要创建跳转脚本：



Windows

在 PATH 目录中创建 to.bat：

@echo off

chcp 936 >nul

if "%1"=="" (

&nbsp;   echo Usage: to ^<alias^>

&nbsp;   exit /b 1

)



for /f "delims=" %%i in ('m to %1') do (

&nbsp;   echo Changing to: %%i

&nbsp;   cd /d "%%i"

)



许可证

MIT License





\*\*2. LICENSE\*\*（MIT 许可证）

```text

MIT License



Copyright (c) 2024 Path-Demo



Permission is hereby granted, free of charge, to any person obtaining a copy

of this software and associated documentation files (the "Software"), to deal

in the Software without restriction, including without limitation the rights

to use, copy, modify, merge, publish, distribute, sublicense, and/or sell

copies of the Software, and to permit persons to whom the Software is

furnished to do so, subject to the following conditions:



The above copyright notice and this permission notice shall be included in all

copies or substantial portions of the Software.



THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR

IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,

FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE

AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER

LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,

OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE

SOFTWARE.







