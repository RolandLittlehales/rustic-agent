/*!
 * Simple HTTP Client Pool for Claude API
 *
 * Provides a shared HTTP client with connection pooling.
 */

use reqwest::Client;
use std::sync::OnceLock;
use std::time::Duration;

/// Global HTTP client instance
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// Simple connection pool that provides a shared HTTP client
#[derive(Debug)]
pub struct ConnectionPool;

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new() -> Self {
        Self
    }

    /// Get the shared HTTP client, creating it if needed
    pub async fn get_client(&self) -> Result<Client, reqwest::Error> {
        // Try to get existing client
        if let Some(client) = HTTP_CLIENT.get() {
            return Ok(client.clone());
        }

        // Create new client with basic pooling
        let client = Client::builder()
            .timeout(Duration::from_secs(crate::config::constants::HTTP_TIMEOUT_SECS))
            .user_agent(crate::config::constants::USER_AGENT)
            .pool_max_idle_per_host(5) // Simple pooling
            .build()?;

        // Try to set the global client (might fail if another thread set it first)
        let _ = HTTP_CLIENT.set(client.clone());
        
        crate::log_debug!("http_client", "Created HTTP client with connection pooling");
        
        Ok(client)
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_pool() {
        let pool = ConnectionPool::new();
        let _client1 = pool.get_client().await.expect("Should create client");
        let _client2 = pool.get_client().await.expect("Should reuse client");
        
        // Test passes if both calls succeed without panicking
    }
}