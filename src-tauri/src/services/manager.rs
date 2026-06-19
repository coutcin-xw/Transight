//! 服务管理器 — 管理翻译服务实例的 CRUD 和启用/禁用

use crate::config::store::{save, ConfigStore, ServiceConfig};

/// 添加服务
pub fn add_service(store: &ConfigStore, service: ServiceConfig) -> Result<Vec<ServiceConfig>, String> {
    let mut config = store.write().map_err(|e| format!("锁失败: {e}"))?;
    config.services.push(service);
    save(&config)?;
    Ok(config.services.clone())
}

/// 更新服务
pub fn update_service(
    store: &ConfigStore,
    id: &str,
    updater: impl FnOnce(&mut ServiceConfig),
) -> Result<Vec<ServiceConfig>, String> {
    let mut config = store.write().map_err(|e| format!("锁失败: {e}"))?;
    if let Some(svc) = config.services.iter_mut().find(|s| s.id == id) {
        updater(svc);
        save(&config)?;
    }
    Ok(config.services.clone())
}

/// 删除服务
pub fn delete_service(store: &ConfigStore, id: &str) -> Result<Vec<ServiceConfig>, String> {
    let mut config = store.write().map_err(|e| format!("锁失败: {e}"))?;
    config.services.retain(|s| s.id != id);
    save(&config)?;
    Ok(config.services.clone())
}

/// 切换服务启用状态
pub fn toggle_service(store: &ConfigStore, id: &str, enabled: bool) -> Result<Vec<ServiceConfig>, String> {
    update_service(store, id, |s| s.enabled = enabled)
}

/// 获取所有服务
pub fn list_services(store: &ConfigStore) -> Result<Vec<ServiceConfig>, String> {
    let config = store.read().map_err(|e| format!("锁失败: {e}"))?;
    Ok(config.services.clone())
}
