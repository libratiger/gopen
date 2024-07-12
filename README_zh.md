# gopen

[English Version](README.md)

`gopen` 是一个命令行工具，用于在浏览器中打开 Git 仓库的远程 URL, 像MacOS内置的`open`命令。

## 功能

- **自动打开远程 URL**：从当前目录或指定目录的 Git 仓库中提取远程 URL，并在浏览器中打开。
- **SSH 转换为 HTTPS**：自动将 SSH URL 转换为 HTTPS URL，以便在浏览器中访问。
- **交互模式**：支持用户通过交互界面选择要打开的多个远程URL中的某一个。
- **目录指定**：用户可以指定一个目录，从该目录下查找并打开远程 URL。

## 安装

你可以通过 Homebrew 安装 `gopen`：

```sh
brew tap libratiger/homebrew-tap
brew install gopen
```

## 使用方法

### 默认模式

在当前 Git 仓库目录中运行 `gopen`，它会自动打开第一个远程 URL。

```sh
gopen
```

### 指定目录

在指定目录下查找并打开远程 URL。

```sh
gopen /path/to/git-repo
```

### 交互模式

使用 `-i` 参数进入交互模式，用户可以选择要打开的远程 URL。

```sh
gopen -i
```

或在指定目录下使用交互模式：

```sh
gopen /path/to/git-repo -i
```

## 示例

```sh
# 默认模式，在当前目录打开远程 URL
gopen

# 指定目录
gopen /path/to/git-repo

# 交互模式
gopen -i

# 指定目录并使用交互模式
gopen /path/to/git-repo -i
```

## 贡献

欢迎提交问题、功能请求或拉取请求。请访问 [GitHub 仓库](https://github.com/libratiger/gopen) 提交您的贡献。

## 许可证

`gopen` 使用 MIT 许可证。
