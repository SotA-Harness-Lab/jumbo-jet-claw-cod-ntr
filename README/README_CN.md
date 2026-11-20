<p align="center">
  <img src="../.github/cod-claw-ntr-logo.svg" alt="CodClaw NTR logo" width="96">
</p>

# CodClaw NTR
<p align="right">
    <a href="../README.md">English</a> | <strong>中文</strong>
</p>

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

> *“小孩子才做选择，成年人我全都要。”*
>
> —— 网络流行语

CodClaw NTR 是 OpenAI Codex CLI 的一个 fork，直接使用你为 Claude Code 准备的配置：`CLAUDE.md`、规则、skill、hook、设置、子代理和 MCP 服务器都在原处读取，一套配置两个工具共用，在 `~/.claude` 里改了，Codex 下一次请求就能用上。

NTR 是 Native Tooling Ready 的缩写：Claude Code 原生的配置可以直接使用，不需要转换或迁移。

## 目录

- [概述](#概述)
- [功能特性](#功能特性)
- [不兼容的部分](#不兼容的部分)
- [安装](#安装)
- [用法](#用法)
- [文档](#文档)
- [免责声明](#免责声明)
- [许可证](#许可证)
- [作者](#作者)

## 概述

CodClaw NTR 保留 Codex 本身（模型、沙箱、审批、TUI 和 IDE 插件），在其上加一层兼容层，到 Claude Code 存放文件的地方直接读取。不复制、不转换，没有同步步骤；`CCN_CLAUDE_COMPAT=0` 可以关掉整个兼容层。Codex 已经有的机制，兼容层直接交给它做，只补上 Codex 没有的部分。逐项的说明见 [Claude Code compatibility](../docs/claude-compat.md)。

改版自己的设置放在 `~/.codex/ccn-config.toml`，不写进 Codex 的 `config.toml`。

## 功能特性

- **指令**：`~/.claude/CLAUDE.md`、项目的 `CLAUDE.md`、`.claude/CLAUDE.md` 和 `CLAUDE.local.md`（支持 `@` 引用），以及 `.claude/rules` 下的规则。带 `paths:` 的规则和子目录里的 `CLAUDE.md`，在读到或改到相应文件时才注入。
- **Skill**：`~/.claude/skills` 和各项目的 `.claude/skills`，支持 Claude Code 的 frontmatter（`disable-model-invocation`、`when_to_use`、`arguments`）。TUI 里用 `/<skill 名> <参数>` 调用，`$ARGUMENTS`、`${CLAUDE_SKILL_DIR}` 等都会替换。
- **Hook**：`settings.json` 里的 hook 照 Claude Code 的方式运行，包括 `Notification`、`PostToolUseFailure`、`StopFailure`，工具名和输入格式也和 Claude Code 一致；`PreToolUse` hook 可以放行、询问或阻止一次调用。
- **设置**：`env` 加进命令的环境变量；Bash 权限规则转成 Codex 的命令规则。
- **自动记忆**：读取项目的 `MEMORY.md`，并且按 Claude Code 的位置和格式写入新记忆，两个工具共用一份记忆。
- **自定义斜杠命令**：`~/.claude/commands`、项目的 `.claude/commands` 和插件的 `commands/` 在原处作为 skill 加载，名字取自路径（`git/commit.md` 就是 `/git:commit`，插件的是 `/<插件名>:<命令名>`）。
- **子代理**：`~/.claude/agents/*.md` 和项目的 `.claude/agents` 变成 agent role。
- **MCP 服务器**：`~/.claude.json` 里的，以及已批准的 `.mcp.json`。
- **插件**：Claude Code 里启用的插件在原处加载；用到 Codex 没有的机制（比如 Claude Code 的状态栏）的插件会被停用，除非在 `ccn-config.toml` 里手动启用。
- **Claude Code 的工具集**（可选，在 `ccn-config.toml` 里设 `[claude] tools = true`）：`Bash`、`Read`、`Write`、`Edit`、`Glob`、`Grep`、`WebFetch`、`WebSearch`、`TodoWrite`、`AskUserQuestion` 和 `Agent` 替换 Codex 自己的工具，作为一层架在它们之上；`Skill` 让模型带参数调用 skill。权限规则（`Read(...)`、`Edit(...)`、`WebFetch(domain:...)`）、子代理的 `tools` 和 hook 的匹配都能和 Claude Code 一一对应。

## 不兼容的部分

- `permissions` 里的 `defaultMode` 和 `additionalDirectories`、企业托管设置、输出风格和状态栏：Codex 没有对应的机制。
- skill 正文里的 shell 注入（`` !`command` ``），以及模型自己打开（而不是通过 `/<名字>` 调用）的 skill 里的替换。
- Claude Code 自己的模型和账号：CodClaw NTR 用的是 Codex 支持的模型。
- Windows：这个 fork 只构建 macOS 和 Linux 版。

## 安装

CodClaw NTR 还没有发布版，需要从源码构建。

### 从源码构建

见 [Installing & building](../docs/install.md)。装好 Rust 后，简单来说：

```shell
cd codex-rs
eval "$(bash ../scripts/fetch-rusty-v8.sh)"
cargo build --release -p codex-cli --bin ccn
cargo build --release -p codex-code-mode-host
```

`scripts/fetch-rusty-v8.sh` 会下载构建时要链接的预编译 V8。

## 用法

在项目里运行 `ccn`。登录方式和 Codex 一样：选 **Sign in with ChatGPT**，或者按 [Codex 的登录文档](https://developers.openai.com/codex/auth#sign-in-with-an-api-key) 配置 API key。

从源码构建的版本没有后台服务的安装包：启动 TUI 时用 `ccn --no-daemon`。

要用 Claude Code 的工具集，新建 `~/.codex/ccn-config.toml`：

```toml
[claude]
tools = true
```

## 文档

- [**Claude Code compatibility**](../docs/claude-compat.md)
- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](../docs/contributing.md)
- [**Installing & building**](../docs/install.md)
- [**Open source fund**](../docs/open-source-fund.md)

## 免责声明

This project is not affiliated with, endorsed by, or sponsored by OpenAI. "Codex" is a trademark of OpenAI and is referenced here solely for descriptive purposes.

This project is not affiliated with, endorsed by, or sponsored by Anthropic. "Claude" and "Claude Code" are trademarks of Anthropic and are referenced here solely for descriptive purposes.

## 许可证

Copyright 2026 Tianyu (Sky) Lu

Except for the third-party code listed below, this repository is 
licensed under the Apache License, Version 2.0 (the "License"); you may 
not use these files except in compliance with the License. You may 
obtain a copy of the License at

```plaintext
http://www.apache.org/licenses/LICENSE-2.0
```

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### Third-Party Code

This repository includes the following third-party code, which remains under its respective original licenses:

| Code | Included As | License | Copyright | Modified |
|------|-------------|---------|-----------|----------|
| [Codex](https://github.com/openai/codex) | Forked from | [Apache-2.0](../LICENSES/Codex-Apache-2.0.txt) | Copyright 2025 OpenAI | Yes |
| [bubblewrap](https://github.com/containers/bubblewrap) | Vendored in `codex-rs/vendor/bubblewrap` | [LGPL-2.0-or-later](../codex-rs/vendor/bubblewrap/COPYING) | Copyright (C) 2016 Alexander Larsson | No |

All other code in this repository, including changes made to the code 
above, is licensed under the Apache License, Version 2.0. The commit history of 
this repository, together with snippets and file headers marked in the 
source files, identifies which code belongs to which project. Files that 
contain code from more than one project are subject to all applicable 
licenses. Contributions to this repository are accepted only under the 
Apache License, Version 2.0 ("inbound = outbound" principle).

## 作者

Tianyu (Sky) Lu
