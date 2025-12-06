use nanoid::nanoid;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn generate_network_id() -> String {
    let id = nanoid!(21, &ALPHABET);
    format!("NET{}", id)
}

pub fn generate_station_id() -> String {
    let id = nanoid!(21, &ALPHABET);
    format!("STA{}", id)
}

pub fn generate_charger_id() -> String {
    let id = nanoid!(21, &ALPHABET);
    format!("CHA{}", id)
}

pub fn generate_connector_id() -> String {
    let id = nanoid!(21, &ALPHABET);
    format!("CON{}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_id() {
        let id = generate_network_id();
        assert!(id.starts_with("NET"));
        assert_eq!(id.len(), 24);
    }

    #[test]
    fn test_station_id() {
        let id = generate_station_id();
        assert!(id.starts_with("STA"));
        assert_eq!(id.len(), 24);
    }

    #[test]
    fn test_charger_id() {
        let id = generate_charger_id();
        assert!(id.starts_with("CHA"));
        assert_eq!(id.len(), 24);
    }

    #[test]
    fn test_connector_id() {
        let id = generate_connector_id();
        assert!(id.starts_with("CON"));
        assert_eq!(id.len(), 24);
    }
}