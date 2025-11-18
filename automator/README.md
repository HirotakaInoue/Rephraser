# Automator Quick Actions セットアップガイド

このガイドでは、Rephraser を macOS の右クリックメニューから使えるようにする方法を説明します。

## 目次

1. [前提条件](#前提条件)
2. [Quick Action の作成（ステップバイステップ）](#quick-action-の作成ステップバイステップ)
3. [動作確認](#動作確認)
4. [複数のアクションの作成](#複数のアクションの作成)
5. [トラブルシューティング](#トラブルシューティング)
6. [カスタマイズ](#カスタマイズ)
7. [アンインストール](#アンインストール)

---

## 前提条件

Quick Action を作成する前に、以下を完了してください：

### 1. Rephraser のインストール

```bash
# プロジェクトディレクトリで
cargo install --path .
```

インストール確認：
```bash
which rephraser
# 出力例: /Users/yourname/.cargo/bin/rephraser
```

### 2. 設定の初期化

```bash
rephraser config init
```

### 3. API キーの設定

**OpenAI を使う場合：**
```bash
# ~/.zshrc または ~/.bash_profile に追加
echo 'export OPENAI_API_KEY="sk-your-api-key-here"' >> ~/.zshrc
source ~/.zshrc
```

**Anthropic を使う場合：**
```bash
echo 'export ANTHROPIC_API_KEY="sk-ant-your-api-key-here"' >> ~/.zshrc
source ~/.zshrc

# 設定ファイルを編集して provider を変更
# ~/.rephraser/config.toml の [llm] セクション:
# provider = "anthropic"
# model = "claude-3-5-sonnet-20241022"
# api_key_env = "ANTHROPIC_API_KEY"
```

### 4. 動作確認

```bash
rephraser rephrase polite "こんにちは"
```

正常に動作することを確認してください。

---

## Quick Action の作成（ステップバイステップ）

ここでは「丁寧に」アクション用の Quick Action を作成します。

### ステップ 1: Automator を開く

1. **Finder** → **アプリケーション** → **Automator.app** を開く
2. またはSpotlight（⌘ + Space）で「Automator」と検索

### ステップ 2: 新しい Quick Action を作成

1. **「新規書類」** をクリック
2. 種類の選択で **「クイックアクション」** を選択
3. **「選択」** をクリック

### ステップ 3: ワークフローの設定

ワークフロー画面の上部で以下を設定：

1. **「ワークフローが受け取る現在の項目：」** → **「テキスト」** を選択
2. **「検索対象：」** → **「任意のアプリケーション」** を選択
3. **「イメージ：」** と **「カラー：」** はデフォルトのまま

### ステップ 4: シェルスクリプトアクションを追加

1. 左側のアクション一覧から **「シェルスクリプトを実行」** を検索
2. **「シェルスクリプトを実行」** をダブルクリックまたはドラッグして右側に追加

### ステップ 5: シェルスクリプトの設定

追加された「シェルスクリプトを実行」アクションで以下を設定：

1. **「シェル：」** → **`/bin/bash`** を選択
2. **「入力の引き渡し方法：」** → **「引数として」** を選択

3. **スクリプト欄に以下を貼り付け：**

```bash
#!/bin/bash

# Automator は通常のシェルセッションではないため、
# 環境変数を読み込むために明示的に shell profile を source する
if [ -f "$HOME/.zshrc" ]; then
    source "$HOME/.zshrc"
fi

if [ -f "$HOME/.bash_profile" ]; then
    source "$HOME/.bash_profile"
fi

# PATH に ~/.cargo/bin を追加
export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH"

# 選択されたテキストを取得（Automator が $1 として渡す）
selected_text="$1"

# Rephraser を実行（polite アクション）
rephraser rephrase polite "$selected_text"
```

### ステップ 6: 保存

1. **⌘ + S** または **「ファイル」** → **「保存」**
2. **名前：** `Rephraser - 丁寧に` （または任意の名前）
3. **「保存」** をクリック

Quick Action が `~/Library/Services/` に保存されます。

---

## 動作確認

### テスト手順

1. **任意のアプリケーションでテキストを選択**
   - TextEdit、Safari、Mail、Slack など

2. **選択したテキストを右クリック**

3. **「サービス」** → **「Rephraser - 丁寧に」** を選択

4. **結果の確認**
   - 設定ファイル（`~/.rephraser/config.toml`）の `output.method` に応じて：
     - `clipboard`: クリップボードにコピーされる（⌘ + V で貼り付け確認）
     - `notification`: 通知が表示される
     - `dialog`: ダイアログが表示される

### 動作しない場合

「[トラブルシューティング](#トラブルシューティング)」セクションを参照してください。

---

## 複数のアクションの作成

他のアクション（「整理する」「要約」など）も同様に作成できます。

### 「整理する」アクションの作成

1. 上記の手順を繰り返す
2. ステップ 5 のスクリプトで `polite` を `organize` に変更：

```bash
#!/bin/bash

# 環境変数を読み込む
if [ -f "$HOME/.zshrc" ]; then
    source "$HOME/.zshrc"
fi

if [ -f "$HOME/.bash_profile" ]; then
    source "$HOME/.bash_profile"
fi

export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH"
selected_text="$1"
rephraser rephrase organize "$selected_text"
```

3. 保存時の名前： `Rephraser - 整理する`

### 「要約」アクションの作成

1. 上記の手順を繰り返す
2. ステップ 5 のスクリプトで `polite` を `summarize` に変更：

```bash
#!/bin/bash

# 環境変数を読み込む
if [ -f "$HOME/.zshrc" ]; then
    source "$HOME/.zshrc"
fi

if [ -f "$HOME/.bash_profile" ]; then
    source "$HOME/.bash_profile"
fi

export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH"
selected_text="$1"
rephraser rephrase summarize "$selected_text"
```

3. 保存時の名前： `Rephraser - 要約`

### カスタムアクションの作成

設定ファイル（`~/.rephraser/config.toml`）に独自のアクションを追加している場合：

```toml
[[actions]]
name = "translate"
display_name = "翻訳"
prompt_template = """
以下のテキストを英語に翻訳してください。

テキスト:
{text}

翻訳:
"""
```

同様に Quick Action を作成し、スクリプトで `translate` を指定します。

---

## トラブルシューティング

### 1. 「rephraser: command not found」エラー

**原因:** シェルの PATH に `~/.cargo/bin` が含まれていない

**解決策:**

スクリプトの先頭に以下が含まれていることを確認：
```bash
export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH"
```

または、`~/.zshrc` に追加して永続化：
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 2. API キーエラー

**エラー例:** `Error: Config("Environment variable 'ANTHROPIC_API_KEY' not found")`

**原因:** Automator は通常のターミナルセッションとは異なり、`~/.zshrc` を自動的に読み込みません。そのため、環境変数が設定されていません。

**解決策:**

**重要：** シェルスクリプト内で明示的に環境変数を読み込む必要があります。

スクリプトの **先頭** に以下が含まれていることを確認してください：

```bash
#!/bin/bash

# Automator は ~/.zshrc を読み込まないため、明示的に source する
if [ -f "$HOME/.zshrc" ]; then
    source "$HOME/.zshrc"
fi

if [ -f "$HOME/.bash_profile" ]; then
    source "$HOME/.bash_profile"
fi
```

そして、API キーが `~/.zshrc` に設定されていることを確認：

```bash
# ~/.zshrc に API キーを追加（まだの場合）
echo 'export ANTHROPIC_API_KEY="sk-ant-your-key"' >> ~/.zshrc
# または
echo 'export OPENAI_API_KEY="sk-your-key"' >> ~/.zshrc

# 確認
source ~/.zshrc
echo $ANTHROPIC_API_KEY
```

**既存の Quick Action を修正する場合:**
1. Automator で該当のワークフローを開く（`~/Library/Services/` から）
2. シェルスクリプト部分を上記のように修正
3. 保存

### 3. Quick Action がメニューに表示されない

**原因1:** macOS がサービスのキャッシュを更新していない

**解決策1:**
```bash
# サービスのキャッシュをクリア
/System/Library/CoreServices/pbs -flush
killall Finder
```

**原因2:** Quick Action の設定が正しくない

**解決策2:**
- Automator でワークフローを開き直す
- 「ワークフローが受け取る現在の項目」が「テキスト」になっているか確認
- 「検索対象」が「任意のアプリケーション」になっているか確認

### 4. 権限エラー

**エラー:** Automation permission denied

**解決策:**
1. **システム設定** → **プライバシーとセキュリティ** → **オートメーション**
2. 使用しているアプリ（TextEdit など）を探す
3. **「Automator」** または **「System Events」** への許可をオンにする

### 5. 結果が表示されない

**clipboard の場合:**
- `pbpaste` で貼り付けて確認
- クリップボード履歴アプリを使用している場合は、そちらも確認

**notification の場合:**
- **システム設定** → **通知** → **「ターミナル」** または **「osascript」**
- 通知を許可しているか確認
- おやすみモードがオフか確認

**dialog の場合:**
- ダイアログがバックグラウンドに隠れていないか確認
- Mission Control（F3）で確認

---

## カスタマイズ

### キーボードショートカットの割り当て

Quick Action にキーボードショートカットを割り当てることができます：

1. **システム設定** → **キーボード** → **ショートカット**
2. 左側で **「サービス」** を選択
3. 右側で **「Rephraser - 丁寧に」** を探す
4. チェックを入れて、ショートカットをクリック
5. 任意のキーの組み合わせを押す（例: ⌃⌥⌘P）

### 出力方法の変更

`~/.rephraser/config.toml` の `[output]` セクションで変更：

```toml
[output]
method = "clipboard"  # または "notification" / "dialog"
```

変更後は即座に反映されます（Quick Action の再作成は不要）。

### アクションの順序変更

システム設定でサービスの表示順序を変更できます：

1. **システム設定** → **キーボード** → **ショートカット** → **サービス**
2. ドラッグして並び替え

---

## アンインストール

### Quick Action の削除

```bash
# すべての Rephraser Quick Action を削除
rm -rf ~/Library/Services/Rephraser*.workflow
```

または Finder で手動削除：
1. Finder で **⌘ + Shift + G** を押す
2. `~/Library/Services/` を入力して移動
3. `Rephraser` で始まるワークフローを削除

### Rephraser 本体のアンインストール

```bash
# バイナリを削除
cargo uninstall rephraser

# 設定ファイルを削除（オプション）
rm -rf ~/.rephraser
```

---

## 追加情報

### 利用可能なアクション一覧

```bash
rephraser list-actions
```

### 設定ファイルの場所

```bash
rephraser config path
```

### 設定の表示

```bash
rephraser config show
```

---

## サポート

問題が解決しない場合は、以下をご確認ください：

1. **ログの確認:**
   ```bash
   # CLI から直接実行してエラーメッセージを確認
   rephraser rephrase polite "テスト"
   ```

2. **GitHub Issues:**
   - バグ報告や機能要望は GitHub Issues へ

3. **設定例:**
   - `examples/example_config.toml` に設定例があります

---

**以上で Quick Actions のセットアップは完了です！**

テキストを選択して右クリックするだけで、Rephraser が使えるようになりました。
