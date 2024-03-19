//! Encoded URLs with a + sign fail

use std::fmt::{Display, Formatter};

use crate::environment::Environment;
use crate::test::{TestGroup, TestGroupResult};

use self::config::Config;

mod config;

/// Encoded URLs with a + sign fail
///
/// An issue was reported where requests that encoded the `+` character in the URL would receive an
/// HTTP 403 Forbidden response. The cause for this issue was that the `+` character has a special
/// meaning in S3, which was not considered when uploading crates in the past. The smoke tests
/// ensure that the Content Delivery Networks correctly rewrite the URL to avoid this issue.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Issue4891 {
    /// Configuration for the test group
    config: Config,
}

impl Issue4891 {
    /// Create a new instance of the test group
    pub fn new(env: Environment) -> Self {
        Self {
            config: Config::for_env(env),
        }
    }
}

impl Display for Issue4891 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rust-lang/crates.io#4891")
    }
}

impl TestGroup for Issue4891 {
    async fn run(&self) -> TestGroupResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::test_utils::*;

    use super::*;

    #[test]
    fn trait_display() {
        let issue_4891 = Issue4891::new(Environment::Staging);

        assert_eq!("rust-lang/crates.io#4891", issue_4891.to_string());
    }

    #[test]
    fn trait_send() {
        assert_send::<Issue4891>();
    }

    #[test]
    fn trait_sync() {
        assert_sync::<Issue4891>();
    }

    #[test]
    fn trait_unpin() {
        assert_unpin::<Issue4891>();
    }
}
