pub fn is_leap_year(year: isize) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 100 != 0 {
        return true;
    }
    
    if year % 400 != 0 {
        return false;
    }

    true
}