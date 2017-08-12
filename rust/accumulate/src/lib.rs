pub fn map_function<F>(input: Vec<i32>, func: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    input.iter().map(|&x| func(x)).collect()
}

pub fn map_closure<F>(input: Vec<i32>, func: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    input.iter().map(|&x| func(x)).collect()
}
