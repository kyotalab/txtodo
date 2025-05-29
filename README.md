# 📝 txtodo – Rust製 todo.txt-cli のミニマル移植

**txtodo** は [todo.txt-cli](https://github.com/todotxt/todo.txt-cli) を参考に、Rustで実装した軽量なタスク管理CLIツールです。
todo.txt 形式の精神を引き継ぎつつ、Rustの実践的なスキルを示すために開発しました。

---

## ✨ プロジェクト概要

このプロジェクトは `todo.txt-cli` の完全な再実装ではなく、
**コア機能だけをRustで実装したミニマルなポート**です。

- Rustの学習・応用のための題材
- 拡張性と保守性を意識した構造
- GitHub上で技術力を示すポートフォリオとしての位置づけ

---

## 🔧 実装済みの機能（MVP）

✅ タスクの追加
✅ 一覧表示
✅ タイトル編集、優先度・期限の設定
✅ 完了フラグの付与
✅ 削除（確認あり）
✅ JSON形式での保存（`todo.json`）

---

## 🛠 インストール

```bash
git clone https://github.com/yourname/txtodo.git
cd txtodo
cargo build --release
cp target/release/txtodo /usr/local/bin/
```

---

## 🚀 使い方

```bash
txtodo <COMMAND>
```

| コマンド | 説明 |
|----------|------|
| `add`    | タスクを追加する（例: `txtodo add "牛乳を買う"`） |
| `pri`    | 優先度を設定（例: `txtodo pri 2 High`） |
| `due`    | 締切を設定（例: `txtodo due 3 2024-06-01`） |
| `mod`    | タイトルを変更する |
| `done`   | 完了済みにする |
| `ls`     | 一覧表示 |
| `del`    | タスクを削除（y/n で確認） |
| `help`   | ヘルプ表示 |

---

## 🧪 使用例

```bash
$ txtodo add "ブログを書く"
Add todo 1: ブログを書く

$ txtodo pri 1 A
1 (A) 2025-05-29 ブログを書く
TODO: 1 prioritized (A).

$ txtodo del 1
Delete 'ブログを書く'? (y/n): y
1 ブログを書く
TODO: 1 deleted.
```

---

## 📁 データ形式

`todo.json` に以下の構造で保存されます：

- `id`: 一意なID
- `title`: タスク内容
- `projects`: プロジェクト
- `contexts`: コンテキスト
- `priority`: 優先度（A / B / C）
- `due_date`: 締切日（任意）
- `created_at`: 作成日
- `end_date`: 完了日
- `done`: 完了フラグ
- `deleted`: 削除フラグ

---

## 🤝 クレジット

本プロジェクトは [`todo.txt-cli`](https://github.com/todotxt/todo.txt-cli) にインスパイアされています。
あくまで参考実装であり、完全な代替を目指すものではありません。

---

## 🧠 作者の意図

> RustでのCLIツール開発を通じて、設計力・実装力を示せる題材を形にしました。
> 小規模ながら、実用性と学習性を兼ね備えたプロジェクトを目指しています。

---