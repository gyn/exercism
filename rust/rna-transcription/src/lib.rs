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
        let mut result = String::with_capacity(self.0.len());

        for c in self.0.chars() {
            match c {
                'A' => result.push('U'),
                'C' => result.push('G'),
                'G' => result.push('C'),
                'T' => result.push('A'),
                _ => return Err("Invalid DNA serial".to_string()),
            }
        }

        Ok(RibonucleicAcid(result))
    }
}
