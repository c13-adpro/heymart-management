pub mod supermarket;

#[cfg(test)]
mod tests {
    use super::supermarket::Supermarket;

    #[test]
    fn test_supermarket() {
        let supermarket = Supermarket {
            id: 1,
            name: "Supermarket".to_string(),
            balance: 100,
            created_at: Some("2021-01-01 00:00:00".to_string()),
        };

        assert_eq!(supermarket.id, 1);
        assert_eq!(supermarket.name, "Supermarket");
        assert_eq!(supermarket.balance, 100);
        assert_eq!(
            supermarket.created_at.unwrap(),
            "2021-01-01 00:00:00".to_string()
        );
    }
}
