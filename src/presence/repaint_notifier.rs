//! Definition of [`RepaintNotifier`].

use std::sync::Arc;

/// Callback the worker invokes after reporting a status, to wake the UI up.
///
/// Using a plain callback keeps the presence module independent from the
/// GUI toolkit.
pub type RepaintNotifier = Arc<dyn Fn() + Send + Sync>;
