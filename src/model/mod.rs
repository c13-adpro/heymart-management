pub mod supermarket;

#[cfg(test)]
mod tests {
    use super::supermarket::Supermarket;
    use sqlx::types::chrono::DateTime;

    #[test]
    fn test_supermarket() {
        let created_at = 1_609_152_000_000_000_000;
        let supermarket = Supermarket {
            id: 1,
            name: "Supermarket".to_string(),
            balance: 100,
            created_at: DateTime::from_timestamp_nanos(created_at),
        };

        assert_eq!(supermarket.id, 1);
        assert_eq!(supermarket.name, "Supermarket");
        assert_eq!(supermarket.balance, 100);
        assert_eq!(
            supermarket.created_at,
            DateTime::from_timestamp_nanos(created_at)
        );
    }
}
