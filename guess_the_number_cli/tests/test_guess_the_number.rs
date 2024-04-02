#[cfg(test)]
mod tests {
    use guess_the_number::features::guess_the_number;

    #[test]
    fn test_guess_the_number () {
        assert_eq!(guess_the_number::search(50, "50".to_string()), 0);
        assert_eq!(guess_the_number::search(50, "49".to_string()), -1);
        assert_eq!(guess_the_number::search(50, "51".to_string()), 1);
    }
}