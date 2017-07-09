#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid(String);

impl RibonucleicAcid {
    pub fn new(serial: &str) -> Self {
        RibonucleicAcid(serial.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid(String);

impl DeoxyribonucleicAcid {
    pub fn new(serial: &str) -> Self {
        DeoxyribonucleicAcid(serial.to_string())
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, String> {
        let rna = self.0
            .chars()
            .filter_map(|c| match c {
                'A' => Some('U'),
                'C' => Some('G'),
                'G' => Some('C'),
                'T' => Some('A'),
                _ => None,
            })
            .collect::<String>();

        if rna.len() != self.0.len() {
            return Err("Invalid DNA serial".to_string());
        }

        Ok(RibonucleicAcid(rna))
    }
}
