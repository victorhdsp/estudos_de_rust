pub fn calcule (value:String) -> Option<f32> {
    return match value.trim().parse::<f32>() {
        Ok(fahrenheit) => Some((fahrenheit - 32.0) / 1.8),
        Err(_) => { 
            println!("Escolha um valor válido!");
            None
        }
    };
}