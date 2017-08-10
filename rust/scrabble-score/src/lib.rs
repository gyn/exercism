#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

const SCORETABLE: [usize; 26] = [
//  a,  b,  c,  d,  e,  f,  g,  h,  i,  j,  k,  l,  m
    1,  3,  3,  2,  1,  4,  2,  4,  1,  8,  5,  1,  3,
//  n,  o,  p,  q,  r,  s,  t,  u,  v,  w,  x,  y,  z
    1,  1,  3, 10,  1,  1,  1,  1,  4,  4,  8,  4, 10,
];

pub fn score(string: &str) -> usize {
    string
        .chars()
        .map(|c| match c {
            _ if c.is_ascii_uppercase() => SCORETABLE[c as usize - 'A' as usize],
            _ if c.is_ascii_lowercase() => SCORETABLE[c as usize - 'a' as usize],
            _ => 0,
        })
        .sum()
}
