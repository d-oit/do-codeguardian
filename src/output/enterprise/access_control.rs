//! Stub access control module for alpha release

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub enable_rbac: bool,
    pub default_deny: bool,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enable_rbac: true,
            default_deny: true,
        }
    }
}

pub struct AccessController;

impl AccessController {
    pub fn new(_config: &AccessControlConfig) -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub async fn validate_access(
        &self,
        _context: &super::EnterpriseContext,
        _permission: &str,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn validate_tenant_access(
        &self,
        _context: &super::EnterpriseContext,
        _tenant_id: uuid::Uuid,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
