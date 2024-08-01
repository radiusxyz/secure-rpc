mod interface;

use std::{any::Any, collections::HashMap, sync::Arc};

pub use interface::context;
use tokio::sync::Mutex;

use crate::error::Error;

pub mod static_str {
    pub static TIME_LOCK_PUZZLE_PARAM: &str = "timelock_puzzle_param";

    pub static KEY_VALIDATION_ZKP_PARAM: &str = "key_validation_zkp_param";
    pub static KEY_VALIDATION_PROVE_KEY: &str = "key_validation_prove_key";
    pub static KEY_VALIDATION_VERIFY_KEY: &str = "key_validation_verify_key";

    pub static POSEIDON_ENCRYPTION_ZKP_PARAM: &str = "poseidon_encryption_zkp_param";
    pub static POSEIDON_ENCRYPTION_PROVE_KEY: &str = "poseidon_encryption_prove_key";
    pub static POSEIDON_ENCRYPTION_VERIFY_KEY: &str = "poseidon_encryption_verify_key";
}

type Value = Box<dyn Any + Send + Sync + 'static>;

/// Represents a thread-safe context for storing and retrieving arbitrary data by string keys.
///
/// Data is stored as a boxed `Any` trait object, which can store any type that implements `Any`.
pub struct Context {
    inner: Arc<Mutex<HashMap<&'static str, Value>>>,
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Default for Context {
    /// Creates an empty `Context`.
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

impl Context {
    /// Asynchronously stores a value in the context associated with a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to associate with the value.
    /// * `value` - The value to be stored.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming usage within an async block.
    /// context().store("my_key", my_value).await;
    /// ```
    pub async fn store<V>(&self, key: &'static str, value: V)
    where
        V: Clone + Send + Sync + 'static,
    {
        let value_any: Value = Box::new(value);

        let mut lock = self.inner.lock().await;

        lock.insert(key, value_any);
    }

    /// Asynchronously retrieves a value from the context by key and tries to downcast it to the desired type.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the value to be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * The key does not exist.
    /// * The stored value cannot be downcasted to the desired type.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming usage within an async block.
    /// let value: MyValue = context().load("my_key").await.unwrap();
    /// ```
    pub async fn load<V>(&self, key: impl AsRef<str>) -> Result<V, Error>
    where
        V: Clone + Send + Sync + 'static,
    {
        let lock = self.inner.lock().await;

        let value_any = lock
            .get(key.as_ref())
            .ok_or(Error::from(Error::KeyDoesNotExist))?;

        match value_any.downcast_ref::<V>() {
            Some(value) => Ok(value.clone()),
            None => Err(Error::Downcast),
        }
    }
}
