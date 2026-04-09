# RTD — 3D Medal Pusher

Bevy + Avian3D による 3D メダル落としゲーム。

## 3D アセット連携方針（Houdini → Bevy）

### エクスポート形式

- **glTF 2.0 (.glb)** を推奨。Bevy の `AssetServer` がネイティブ対応しており、FBX/ABC より安定。
- Houdini から直接 glTF を出す場合は SideFX Labs の `glTF ROP` を使用。
- FBX 経由の場合は Blender 等で glTF に変換する中間ステップが必要。

### アセット一覧（予定）

| アセット | ファイル名（仮） | 役割 |
|---------|-----------------|------|
| トレイ   | `assets/tray.glb` | 床・壁・スロット穴を含む筐体 |
| プッシャー | `assets/pusher.glb` | 往復するプッシャー板 |
| メダル   | `assets/medal.glb` | 投入されるコイン |

### スロット位置の伝達方法

Houdini 側でスロットの位置を自由に決め、それを Rust 側に伝える方法は以下の 2 つ。

#### 方法 A: Empty / Locator マーカー（推奨）

1. Houdini でトレイモデル内に **Null ノード**（名前: `slot_left`, `slot_right`）を配置
2. glTF エクスポート時にこれらは **空のノード（Empty）** として出力される
3. Bevy 側で `SceneBundle` をロード後、名前でノードを検索して位置を取得:

```rust
fn extract_slot_positions(
    children: Query<(&Name, &GlobalTransform)>,
) {
    for (name, tf) in &children {
        match name.as_str() {
            "slot_left" => { /* tf.translation() がスロット位置 */ }
            "slot_right" => { /* tf.translation() がスロット位置 */ }
            _ => {}
        }
    }
}
```

4. 取得した位置を `Resource` に格納し、メダルスポーンで参照する

#### 方法 B: glTF Extras（JSON メタデータ）

1. Houdini / Blender で glTF の `extras` フィールドに JSON を埋め込む:
   ```json
   { "slot_left_z": -245.0, "slot_right_z": 245.0, "slot_y": -140.0 }
   ```
2. Bevy 側で `GltfExtras` コンポーネントからデシリアライズ:
   ```rust
   fn read_extras(query: Query<&GltfExtras>) {
       for extras in &query {
           let v: serde_json::Value = serde_json::from_str(&extras.value).unwrap();
           // v["slot_left_z"] etc.
       }
   }
   ```

### 移行手順

1. **Phase 1（現在）**: `config.rs` のハードコード定数でプロトタイプ
2. **Phase 2**: Houdini でアセット作成、Empty マーカーでスロット位置を埋め込み
3. **Phase 3**: Bevy 側で `AssetServer::load()` → シーンスポーン → マーカー検索 → 定数を `Resource` に置換

```rust
// Phase 3 のロードイメージ
fn load_tray(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load("tray.glb#Scene0")));
}
```

### Houdini 側の制約メモ

- Bevy の座標系は **Y-up, 右手系**。Houdini もデフォルト Y-up なのでそのまま。
- スケールは **1 unit = 1 px**（現在の config.rs 基準）。アセット側のスケールを合わせるか、Bevy 側で `Transform::from_scale()` で調整。
- コライダーはメッシュ形状から自動生成するか、簡易形状（Box/Cylinder）を `config.rs` に残すか選択。性能面では簡易形状を推奨。
