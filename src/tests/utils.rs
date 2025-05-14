#[cfg(test)]
mod utils {
    use crate::utils::utils::generate_ic;

    #[tokio::test]
    async fn check_nanoid(){
    let nanoid = generate_ic();
        assert_eq!(nanoid.to_string().len(), 9);
    }
}