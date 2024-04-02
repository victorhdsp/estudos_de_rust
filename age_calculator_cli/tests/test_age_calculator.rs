#[cfg(test)]
mod tests {
    use age_calculator::features::age_calculator;
    
    #[test]
    fn test_calcule_age () {
        assert_eq!(age_calculator::calcule("1998".to_string(), "12".to_string()), Some(25));
        assert_eq!(age_calculator::calcule("2002".to_string(), "4".to_string()), Some(22));
        assert!(age_calculator::calcule("1998".to_string(), "13".to_string()).is_none());
        assert!(age_calculator::calcule("1998".to_string(), "0".to_string()).is_none());
    }
}