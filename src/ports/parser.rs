use super::ParsePortError;

pub fn parse_ports(ports: &str) -> Result<Vec<u16>, ParsePortError> {
    if ports.is_empty() {
        return Err(ParsePortError::EmptyInput);
    }

    let mut result = vec![];

    for el in ports.split(',') {
        let el = el.trim();

        if let Some((start_str, end_str)) = el.split_once('-') {
            let start = parse_port(start_str)?;
            let end = parse_port(end_str)?;

            if start > end {
                return Err(ParsePortError::InvalidRange);
            }

            result.extend(start..=end);
        } else {
            result.push(parse_port(el)?);
        }
    }
    Ok(result)
}

fn parse_port(port: &str) -> Result<u16, ParsePortError> {
    port.parse::<u16>().map_err(|_| ParsePortError::InvalidPort)
}
