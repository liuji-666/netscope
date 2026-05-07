/// 检测模块的返回状态
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ProbeStatus {
    Ok,
    Partial,
    Error,
    Timeout,
    Skipped,
}

/// 通用检测结果包装
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProbeResult<T: serde::Serialize> {
    pub status: ProbeStatus,
    pub data: Option<T>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

impl<T: serde::Serialize> ProbeResult<T> {
    pub fn ok(data: T, duration: std::time::Duration) -> Self {
        Self {
            status: ProbeStatus::Ok,
            data: Some(data),
            error: None,
            duration_ms: duration.as_millis() as u64,
        }
    }

    pub fn error(err: &str, duration: std::time::Duration) -> Self {
        Self {
            status: ProbeStatus::Error,
            data: None,
            error: Some(err.to_string()),
            duration_ms: duration.as_millis() as u64,
        }
    }

    pub fn skipped(reason: &str) -> Self {
        Self {
            status: ProbeStatus::Skipped,
            data: None,
            error: Some(reason.to_string()),
            duration_ms: 0,
        }
    }
}