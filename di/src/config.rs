use std::sync::Arc;
use std::sync::RwLock;


// グローバルな設定内容を定義
#[derive(Default)]
pub struct Config {
    pub debug_mode: bool,
}
impl Config {
    // 設定の参照
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    // 設定を上書き
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}
thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}


