pub fn private_key(p: u64) -> u64 {
    p - 1
}

fn modpow(b: u64, e: u64, m: u64) -> u64 {
    if m == 1 {
        0
    } else {
        let mut r = 1;
        let mut me = e;
        let mut mb = b % m;

        while me > 0 {
            if me % 2 == 1 {
                r = (r * mb) % m
            }
            me = me >> 1;
            mb = (mb * mb) % m
        }

        r
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modpow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modpow(b_pub, a, p)
}
