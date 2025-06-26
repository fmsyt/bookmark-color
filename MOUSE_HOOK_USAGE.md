# マウスフック機能使用法

この実装では、WindowsのSetWindowsHookExWを使用してシステムレベルでマウスクリックイベントを検知します。

## 主な機能

### Rustレベル

- `Watcher`構造体でマウスフックを管理
- 左クリック、右クリック、中クリックを検知
- イベントキューでクリック情報を保存

### Tauriコマンド

- `start_mouse_hook()`: マウスフックを開始
- `stop_mouse_hook()`: マウスフックを停止
- `is_mouse_hook_running()`: フック状態を確認
- `get_mouse_events()`: 蓄積されたクリックイベントを取得

## フロントエンドでの使用例

```javascript
// マウスフックを開始
const isStarted = await invoke('start_mouse_hook');
console.log('フック開始:', isStarted);

// 定期的にイベントをチェック
const checkEvents = async () => {
    const events = await invoke('get_mouse_events');
    events.forEach(event => {
        console.log(`${event.button}クリック at (${event.point.x}, ${event.point.y})`);
    });
};

// 1秒ごとにイベントをチェック
setInterval(checkEvents, 1000);

// フックを停止
const isStopped = await invoke('stop_mouse_hook');
console.log('フック停止:', isStopped);
```

## 注意事項

1. **管理者権限**: システムレベルのフックには管理者権限が必要な場合があります
2. **パフォーマンス**: 全マウスイベントを監視するため、CPU使用率に注意
3. **メモリ管理**: イベントキューは自動的に1000件で制限されます
4. **Windows専用**: この実装はWindows専用です

## セキュリティ考慮事項

- ユーザーのマウス操作を監視するため、適切な通知と同意が必要
- 不要な情報の収集を避け、プライバシーを保護してください
