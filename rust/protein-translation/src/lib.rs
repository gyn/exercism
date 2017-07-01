use std::str;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProteinMap<'a> {
    stat: HashMap<&'a str, &'a str>,
}

impl<'a> ProteinMap<'a> {
    pub fn name_for(&self, rna: &str) -> Result<&str, &str> {
        self.stat.get(rna).map(|r| *r).ok_or("Invailid codons")
    }

    pub fn of_rna(&self, rna: &str) -> Result<Vec<&str>, &str> {
        if rna.len() % 3 != 0 {
            return Err("Invalid length");
        }

        let r = rna.as_bytes()
            .chunks(3)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| str::from_utf8(x).unwrap())
            .take_while(|&x| x != "UAA" && x != "UAG" && x != "UGA")
            .map(|x| self.name_for(x))
            .collect::<Vec<_>>();

        if r.iter().any(|&x| x.is_err()) {
            return Err("Invalid codons");
        }

        Ok(r.iter().map(|x| x.unwrap()).collect())
    }
}

pub fn parse(v: Vec<(&'static str, &'static str)>) -> ProteinMap {
    ProteinMap { stat: v.iter().map(|&(a, b)| (a, b)).collect() }
}
