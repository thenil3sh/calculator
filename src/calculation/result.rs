pub trait CalculationResult {
    fn approximate(&self) -> String;
}

impl CalculationResult for f64 {
    fn approximate(&self) -> String {
        if self.to_string().len() >= 12 {
            let string = format!("{self:1.9e}");
            let e_pos = string.find('e').unwrap();
            format!(
                "{}{}",
                string[..e_pos].trim_end_matches('0'),
                &string[e_pos..]
            )
        } else {
            self.to_string()
        }
    }
}
