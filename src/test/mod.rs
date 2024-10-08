//! Types that represent tests and their results

use async_trait::async_trait;

pub use self::test_group::TestGroup;
pub use self::test_group_result::TestGroupResult;
pub use self::test_result::TestResult;
pub use self::test_suite::TestSuite;
pub use self::test_suite_result::TestSuiteResult;

mod test_group;
mod test_group_result;
mod test_result;
mod test_suite;
mod test_suite_result;

/// A test
///
/// A test performs a single check against the Rust project's infrastructure. It returns a result
/// that indicates whether the check passed or failed. Tests should be idempotent and should not
/// have side effects.
#[async_trait]
pub trait Test: Send + Sync {
    /// Run the test
    async fn run(&self) -> TestResult;
}
