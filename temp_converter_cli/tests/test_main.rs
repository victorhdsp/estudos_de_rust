#[cfg(test)]
mod tests {
    use temp_converter::features::fahrenheit_to_celsius;
    
    #[test]
    fn test_convert_fahrenheit_to_celsius () {
        assert_eq!(fahrenheit_to_celsius::calcule("0".to_string()).unwrap(),  -17.777779);
        assert_eq!(fahrenheit_to_celsius::calcule("32".to_string()).unwrap(), 0.0);
        assert_eq!(fahrenheit_to_celsius::calcule("50".to_string()).unwrap(), 10.0);
        assert_eq!(fahrenheit_to_celsius::calcule("104".to_string()).unwrap(), 40.0);
    }
}