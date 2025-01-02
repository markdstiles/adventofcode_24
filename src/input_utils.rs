pub fn parse_formatted(input: String, format: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    if format == "{}" {
        result.push(input);
        return result
    }

    let mut from: Option<usize> = None;
    let mut input_str = input.as_str();

    for split in format.split("{}") {
        if from.is_none() {
            from = input_str.find(split);

            if let Some(f) = from {
                from = Some(f + split.len());
            }
        } else if let Some(f) = from {
            let to = input_str.find(split);
            
            if split.is_empty() {
                result.push(input_str[f..].into());
                break;
            }

            if let Some(t) = to {
                result.push(input_str[f..t].into());
                input_str = &input_str[t..];
                from = Some(split.len());
            }
            else {result.push(input_str[f..].into());
                break;
            }
        }
    }

    result
}