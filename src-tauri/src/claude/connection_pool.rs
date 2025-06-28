/*!
 * HTTP Connection Pool for Claude API
 *
 * Implements connection pooling to reduce overhead by reusing HTTP connections.
 * This should reduce latency by ~50% by avoiding TCP handshake overhead.
 */

use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of idle connections per host
    pub max_idle_per_host: usize,
    /// Timeout for establishing connections
    pub connect_timeout: Duration,
    /// Timeout for idle connections before cleanup
    pub idle_timeout: Duration,
    /// Total timeout for requests
    pub request_timeout: Duration,
    /// User agent string
    pub user_agent: String,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_idle_per_host: 10,
            connect_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(60),
            request_timeout: Duration::from_secs(crate::config::constants::HTTP_TIMEOUT_SECS),
            user_agent: crate::config::constants::USER_AGENT.to_string(),
        }
    }
}

/// HTTP connection pool manager
#[derive(Debug)]
pub struct ConnectionPool {
    config: PoolConfig,
    client: Arc<RwLock<Option<Client>>>,
}

impl ConnectionPool {
    /// Create a new connection pool with default configuration
    pub fn new() -> Self {
        Self::with_config(PoolConfig::default())
    }

    /// Create a new connection pool with custom configuration
    pub fn with_config(config: PoolConfig) -> Self {
        Self {
            config,
            client: Arc::new(RwLock::new(None)),
        }
    }

    /// Get or create the pooled HTTP client
    pub async fn get_client(&self) -> Result<Client, reqwest::Error> {
        // Try to get existing client first (fast path)
        {
            let client_guard = self.client.read().await;
            if let Some(client) = client_guard.as_ref() {
                return Ok(client.clone());
            }
        }

        // Create new client if needed (slow path)
        let mut client_guard = self.client.write().await;

        // Double-check pattern: another thread might have created the client
        if let Some(client) = client_guard.as_ref() {
            return Ok(client.clone());
        }

        // Create the new client with optimized settings
        let client = Client::builder()
            .pool_max_idle_per_host(self.config.max_idle_per_host)
            .pool_idle_timeout(self.config.idle_timeout)
            .connect_timeout(self.config.connect_timeout)
            .timeout(self.config.request_timeout)
            .user_agent(&self.config.user_agent)
            .tcp_keepalive(Duration::from_secs(30))
            .tcp_nodelay(true)
            .build()?;

        crate::log_info!(
            "connection_pool",
            "Created new HTTP client with connection pooling",
            {
                let mut context = std::collections::HashMap::new();
                context.insert(
                    "max_idle_per_host".to_string(),
                    self.config.max_idle_per_host.to_string(),
                );
                context.insert(
                    "idle_timeout_secs".to_string(),
                    self.config.idle_timeout.as_secs().to_string(),
                );
                context.insert(
                    "connect_timeout_secs".to_string(),
                    self.config.connect_timeout.as_secs().to_string(),
                );
                context
            }
        );

        *client_guard = Some(client.clone());
        Ok(client)
    }

    /// Reset the connection pool (useful for configuration changes)
    pub async fn reset(&self) {
        let mut client_guard = self.client.write().await;
        *client_guard = None;

        crate::log_info!("connection_pool", "Connection pool reset");
    }

    /// Get pool statistics for monitoring
    pub async fn get_stats(&self) -> PoolStats {
        let client_guard = self.client.read().await;
        PoolStats {
            is_initialized: client_guard.is_some(),
            config: self.config.clone(),
        }
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub is_initialized: bool,
    pub config: PoolConfig,
}

impl PoolStats {
    /// Format stats for logging/monitoring
    pub fn format(&self) -> String {
        format!(
            "ConnectionPool(initialized={}, max_idle={}, timeouts=connect:{}s,idle:{}s,request:{}s)",
            self.is_initialized,
            self.config.max_idle_per_host,
            self.config.connect_timeout.as_secs(),
            self.config.idle_timeout.as_secs(),
            self.config.request_timeout.as_secs()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_connection_pool_creation() {
        let pool = ConnectionPool::new();
        let _client = pool.get_client().await.expect("Should create client");

        // Getting client again should reuse the same instance
        let _client2 = pool.get_client().await.expect("Should reuse client");

        // Note: We can't directly compare Client instances for equality,
        // but we can verify the pool is initialized
        let stats = pool.get_stats().await;
        assert!(stats.is_initialized);
    }

    #[tokio::test]
    async fn test_pool_reset() {
        let pool = ConnectionPool::new();
        let _client = pool.get_client().await.expect("Should create client");

        let stats_before = pool.get_stats().await;
        assert!(stats_before.is_initialized);

        pool.reset().await;

        let stats_after = pool.get_stats().await;
        assert!(!stats_after.is_initialized);
    }

    #[tokio::test]
    async fn test_custom_config() {
        let config = PoolConfig {
            max_idle_per_host: 5,
            connect_timeout: Duration::from_secs(5),
            idle_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            user_agent: "test-agent".to_string(),
        };

        let pool = ConnectionPool::with_config(config.clone());
        let _client = pool.get_client().await.expect("Should create client");

        let stats = pool.get_stats().await;
        assert_eq!(stats.config.max_idle_per_host, 5);
        assert_eq!(stats.config.connect_timeout, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_concurrent_client_creation() {
        use std::sync::Arc;
        
        let pool = Arc::new(ConnectionPool::new());
        let mut handles = vec![];
        
        // Spawn multiple tasks trying to get clients concurrently
        for _ in 0..10 {
            let pool_clone = pool.clone();
            let handle = tokio::spawn(async move {
                pool_clone.get_client().await
            });
            handles.push(handle);
        }
        
        // All should succeed
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
        
        // Pool should be initialized only once
        let stats = pool.get_stats().await;
        assert!(stats.is_initialized);
    }

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.max_idle_per_host, 10);
        assert_eq!(config.connect_timeout, Duration::from_secs(10));
        assert_eq!(config.idle_timeout, Duration::from_secs(60));
        assert_eq!(config.user_agent, crate::config::constants::USER_AGENT);
    }

    #[test]
    fn test_pool_stats_format() {
        let config = PoolConfig::default();
        let stats = PoolStats {
            is_initialized: true,
            config,
        };
        
        let formatted = stats.format();
        assert!(formatted.contains("ConnectionPool"));
        assert!(formatted.contains("initialized=true"));
        assert!(formatted.contains("max_idle=10"));
    }
}
