//! Stub compliance module for alpha release

use super::{ComplianceFramework, Tenant};
use crate::types::AnalysisResults;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ComplianceReporter;

impl ComplianceReporter {
    pub fn new(_config: &super::EnterpriseConfig) -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub fn disabled() -> Self {
        Self
    }

    pub async fn generate_compliance_data(
        &self,
        _results: &AnalysisResults,
    ) -> anyhow::Result<ComplianceData> {
        Ok(ComplianceData {
            framework: ComplianceFramework::SOC2,
            generated_at: Utc::now(),
            data: serde_json::Value::Null,
        })
    }

    pub async fn generate_report(
        &self,
        framework: &ComplianceFramework,
        _tenant: Option<&Tenant>,
    ) -> anyhow::Result<ComplianceReport> {
        Ok(ComplianceReport {
            id: Uuid::new_v4(),
            framework: framework.clone(),
            generated_at: Utc::now(),
            content: "Stub compliance report".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ComplianceData {
    pub framework: ComplianceFramework,
    pub generated_at: DateTime<Utc>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub id: Uuid,
    pub framework: ComplianceFramework,
    pub generated_at: DateTime<Utc>,
    pub content: String,
}
