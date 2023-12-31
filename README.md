# mono-search

## 概要

Unixコマンドの `find` と `grep` を組み合わせたようなCLIファイル検索ツールです．複数ディレクトリを対象に，正規表現に合致する行を有するファイルまたは特定のファイル名を持つファイルを検索できます．

`find` と `grep` をパイプでつなげるなどして複雑なシェルスクリプトを書くのに比べて，ずっと直感的にわかりやすく操作することができます．

## 機能

* ファイル名による検索
  * 指定されたディレクトリの中を再帰的にチェックし，特定のファイル名のファイルを検索することができる
  * 検索にヒットしたファイルのパスを列挙して出力する
  * ヒットしたファイルの数を数えて表示する

* 複数ディレクトリ検索
  * 「ファイル名による検索」を複数のディレクトリに対して一度の実行で行うことができる

* gitリポジトリ対応
  * 検索対象のディレクトリがgit管理されている場合は，実行時のカレントブランチに関わらず `main` または `master` ブランチにおいて検索を行う
  * ブランチの選択は「あらかじめ決めたブランチ」「`main` または `master`」「そのときのブランチ」の３つから選択できる

* 正規表現によるテキスト検索
  * 指定されたディレクトリの中を再帰的にチェックし，与えられた正規表現に合致するような行があるファイルを検索することができる
  * 「ファイル名による検索」と「正規表現によるテキスト検索」は同時にできる

* UIなど
  * IDE上で行うファイル検索よりも高速に動作する
  * 検索作業の進捗を表示する
  * 以前の検索条件を記憶して，再度適用することができる
  * 依存するツールのインストールなどが不要で，単独で使用できる

## 参考にしたもの

* [Command Line Applications in Rust](https://rust-cli.github.io/book/) Rust でCLIツールを作ることに関する本.
* [fd-find](https://github.com/sharkdp/fd) Rustによる高速な `find` の再実装．ライブラリとしてCLIツールの内部で使えればよかったが，それはできない．
* [ignore](https://crates.io/crates/ignore) fd-findの内部で使用されているファイル検索クレート．
* [clap](https://crates.io/crates/clap) コマンドラインツールを作るためのクレート．
* [tempdir](https://crates.io/crates/tempdir) 一時ファイルを作成することができる，テストを楽にしてくれそうなクレート.
