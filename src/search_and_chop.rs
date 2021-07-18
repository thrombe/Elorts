
pub fn search_and_chop<'a>(text: &'a str, find_start: &str, find_end: &str) -> Vec<&'a str> {
    let mut found_vec = Vec::new();
    let mut curr_text = &text[..];
    while curr_text.contains(find_start) && curr_text.contains(find_end) {
        let findex = curr_text.find(find_start).unwrap()+find_start.len();
        curr_text = &curr_text[findex..];
        let eindex = curr_text.find(find_end).unwrap();
        found_vec.push(&curr_text[..eindex]);
        curr_text = &curr_text[eindex+1..]
    }
    found_vec
}
