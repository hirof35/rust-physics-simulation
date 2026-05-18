# Rust 3D Physics Chaos Simulation 📦💥

Rustの高速な3Dゲームエンジン **Bevy** と、物理エンジン **Rapier3D** を使用した、100個のオブジェクトによる超高速・高パフォーマンスな物理衝突シミュレーションです。
<img width="1595" height="941" alt="スクリーンショット 2026-05-19 064258" src="https://github.com/user-attachments/assets/05941ea0-df5d-419f-9666-5a85852f4ad5" />

スペースキーを押すことで、物理エンジンの内部データを完全にリセット（デタッチ＆リスポーン方式）し、何度でも上空からカオスな崩壊をやり直すことができます。

## 🛠️ 動作環境 / 開発環境
- **OS:** Windows 11 / macOS / Linux
- **Language:** Rust (Edition 2021)
- **Engine:** Bevy 0.13
- **Physics:** Bevy Rapier3D 0.25

## 🚀 起動方法

リポジトリをクローンし、以下のコマンドを実行してください。

```bash
git clone [https://github.com/あなたのユーザー名/リポジトリ名.git](https://github.com/あなたのユーザー名/リポジトリ名.git)
cd リポジトリ名
cargo run --release
