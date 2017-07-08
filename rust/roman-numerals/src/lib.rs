#[derive(Debug)]
pub struct Roman(u32);

impl From<u32> for Roman {
    #[inline]
    fn from(v: u32) -> Roman {
        Roman(v)
    }
}

const ROMAN_DIGITS: [(u32, [&'static str; 10]); 4] =
    [
        (
            1000,
            ["", "M", "MM", "MMM", "", "", "", "", "", ""],
        ),
        (
            100,
            ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
        ),
        (
            10,
            ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
        ),
        (
            1,
            ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
        ),
    ];

impl ToString for Roman {
    #[inline]
    fn to_string(&self) -> String {
        assert!(self.0 <= 3000);

        let mut digit = self.0;
        let mut result = String::new();

        for v in &ROMAN_DIGITS {
            let index = digit / v.0;

            result.push_str(v.1[index as usize]);

            digit -= index * v.0;
        }

        result
    }
}
