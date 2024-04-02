fn parse_selected (_value: String) -> Option<u32> {
    return match _value.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Escolha um valor válido");
            None
        }
    }
}

pub fn search (expected: u32, _selected: String) -> i32 {
    let selected = parse_selected(_selected).unwrap();

    if selected > expected {
        return 1;
    } else if selected < expected {
        return -1;
    }
    
    return 0 as i32;
}