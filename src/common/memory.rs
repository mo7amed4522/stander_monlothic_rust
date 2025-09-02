//! Memory optimization and garbage collection utilities

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::HashMap;
use tracing::{info, warn, debug};


#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub allocated_bytes: usize,
    pub peak_allocated_bytes: usize,
    pub gc_runs: u64,
    pub last_gc_time: Option<Instant>,
}


#[derive(Debug, Clone)]
pub struct GcConfig {
    pub max_memory_mb: usize,
    pub gc_interval_seconds: u64,
    pub force_gc_threshold_mb: usize,
    pub enable_auto_gc: bool,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            gc_interval_seconds: 300,
            force_gc_threshold_mb: 256,
            enable_auto_gc: true,
        }
    }
}


#[derive(Debug)]
pub struct MemoryManager {
    config: GcConfig,
    stats: Arc<RwLock<MemoryStats>>,
    cache: Arc<RwLock<HashMap<String, (Vec<u8>, Instant)>>>,
}

impl MemoryManager {
    pub fn new(config: GcConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(MemoryStats {
                allocated_bytes: 0,
                peak_allocated_bytes: 0,
                gc_runs: 0,
                last_gc_time: None,
            })),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_auto_gc(&self) {
        if !self.config.enable_auto_gc {
            return;
        }

        let stats = Arc::clone(&self.stats);
        let cache = Arc::clone(&self.cache);
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(config.gc_interval_seconds));

            loop {
                interval.tick().await;

                let current_memory = {
                    let stats_guard = stats.read().await;
                    stats_guard.allocated_bytes
                };

                let threshold_bytes = config.force_gc_threshold_mb * 1024 * 1024;

                if current_memory > threshold_bytes {
                    info!("Auto GC triggered: current memory {}MB > threshold {}MB",
                          current_memory / 1024 / 1024,
                          config.force_gc_threshold_mb);

                    Self::run_gc_internal(&stats, &cache).await;
                }
            }
        });
    }
    pub async fn force_gc(&self) {
        info!("Manual GC triggered");
        Self::run_gc_internal(&self.stats, &self.cache).await;
    }
    pub async fn cache_data(&self, key: String, data: Vec<u8>) {
        let data_size = data.len();
        {
            let mut cache_guard = self.cache.write().await;
            cache_guard.insert(key, (data, Instant::now()));
        }
        {
            let mut stats_guard = self.stats.write().await;
            stats_guard.allocated_bytes += data_size;
            if stats_guard.allocated_bytes > stats_guard.peak_allocated_bytes {
                stats_guard.peak_allocated_bytes = stats_guard.allocated_bytes;
            }
        }
        debug!("Cached data: {} bytes, total allocated: {} bytes",
               data_size,
               self.stats.read().await.allocated_bytes);
    }
    pub async fn get_cached_data(&self, key: &str) -> Option<Vec<u8>> {
        let cache_guard = self.cache.read().await;
        cache_guard.get(key).map(|(data, _)| data.clone())
    }
    pub async fn remove_cached_data(&self, key: &str) -> bool {
        let mut cache_guard = self.cache.write().await;
        if let Some((data, _)) = cache_guard.remove(key) {
            let mut stats_guard = self.stats.write().await;
            stats_guard.allocated_bytes = stats_guard.allocated_bytes.saturating_sub(data.len());
            true
        } else {
            false
        }
    }
    pub async fn get_stats(&self) -> MemoryStats {
        self.stats.read().await.clone()
    }
    pub async fn clear_cache(&self) {
        let mut cache_guard = self.cache.write().await;
        cache_guard.clear();

        let mut stats_guard = self.stats.write().await;
        stats_guard.allocated_bytes = 0;

        info!("Cache cleared");
    }
    async fn run_gc_internal(
        stats: &Arc<RwLock<MemoryStats>>,
        cache: &Arc<RwLock<HashMap<String, (Vec<u8>, Instant)>>>,
    ) {
        let start_time = Instant::now();
        let mut freed_bytes = 0;
        let mut removed_items = 0;
        let expiry_threshold = Instant::now() - Duration::from_secs(3600);
        {
            let mut cache_guard = cache.write().await;
            let keys_to_remove: Vec<String> = cache_guard
                .iter()
                .filter(|(_, (_, timestamp))| *timestamp < expiry_threshold)
                .map(|(key, _)| key.clone())
                .collect();
            for key in keys_to_remove {
                if let Some((data, _)) = cache_guard.remove(&key) {
                    freed_bytes += data.len();
                    removed_items += 1;
                }
            }
        }
        {
            let mut stats_guard = stats.write().await;
            stats_guard.allocated_bytes = stats_guard.allocated_bytes.saturating_sub(freed_bytes);
            stats_guard.gc_runs += 1;
            stats_guard.last_gc_time = Some(start_time);
        }
        let duration = start_time.elapsed();
        info!(
            "GC completed: freed {}MB in {} items, took {:?}",
            freed_bytes / 1024 / 1024,
            removed_items,
            duration
        );
    }
}


static MEMORY_MANAGER: tokio::sync::OnceCell<MemoryManager> = tokio::sync::OnceCell::const_new();

pub async fn init_memory_manager(config: Option<GcConfig>) -> &'static MemoryManager {
    MEMORY_MANAGER
        .get_or_init(|| async {
            let manager = MemoryManager::new(config.unwrap_or_default());
            manager.start_auto_gc().await;
            manager
        })
        .await
}
pub async fn get_memory_manager() -> Option<&'static MemoryManager> {
    MEMORY_MANAGER.get()
}
pub async fn force_gc() {
    if let Some(manager) = get_memory_manager().await {
        manager.force_gc().await;
    } else {
        warn!("Memory manager not initialized");
    }
}
pub async fn get_memory_stats() -> Option<MemoryStats> {
    if let Some(manager) = get_memory_manager().await {
        Some(manager.get_stats().await)
    } else {
        None
    }
}
