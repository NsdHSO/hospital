#[cfg(test)]
mod utils {
    use crate::utils::helpers::generate_ic;

    #[tokio::test]
    async fn check_nanoid() {
        let nanoid = generate_ic();
        // The length of the string representation of the i32 may vary
        // but should not exceed 9 characters
        assert!(nanoid.to_string().len() <= 9);
    }
}
