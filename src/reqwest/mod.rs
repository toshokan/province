#[cfg(feature = "sync")]
pub mod sync;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        use sync::Executor;
        let exec = Executor::new();
        crate::tests::ip_test_execute(exec).unwrap()
    }
}
