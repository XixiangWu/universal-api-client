# 推送到 GitHub 指南

## 步骤 1: 在 GitHub 创建新仓库

1. 访问 https://github.com/new
2. 仓库名称: `universal-api-client`
3. 描述: `Lightweight Rust library for unified API access to Anthropic and OpenAI services`
4. 选择 **Public** 或 **Private**
5. **不要** 勾选 "Initialize this repository with a README"
6. 点击 "Create repository"

## 步骤 2: 推送代码

复制 GitHub 显示的仓库 URL，然后执行：

```bash
# 添加远程仓库（替换为你的用户名）
git remote add origin https://github.com/你的用户名/universal-api-client.git

# 推送代码
git push -u origin main
```

## 步骤 3: 验证

访问你的 GitHub 仓库页面，应该能看到：
- ✅ README.md 正确显示
- ✅ 所有文件已上传
- ✅ 提交历史完整

## 步骤 4: 更新文档中的链接

在 GitHub 仓库创建后，更新以下文件中的 `你的用户名` 占位符：
- README.md
- INSTALL.md
- QUICKSTART.md

然后提交并推送：

```bash
git add -A
git commit -m "docs: update GitHub username in documentation"
git push
```

## 完成！

现在你可以在任何地方使用这个仓库：

```bash
git clone https://github.com/你的用户名/universal-api-client.git
cd universal-api-client
```

在 Claude Code 中说：

```
请按照 docs/plans/implementation.md 实现通用 API 客户端库
```

---

## 快速命令（复制粘贴）

```bash
# 1. 添加远程仓库（记得替换用户名！）
git remote add origin https://github.com/你的用户名/universal-api-client.git

# 2. 推送
git push -u origin main

# 3. 验证
git remote -v
```
