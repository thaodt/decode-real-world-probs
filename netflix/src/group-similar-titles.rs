use std::collections::HashMap;

//TODO: rewrite this fn with arguments of Vec<&str> instead
fn group_titles(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut res = HashMap::new();
    let mut count = vec![0; 26]; // 26 English alphabets

    for s in strs.into_iter() {
        for c in s.chars() {
            let index = (c as u32 - 'a' as u32) as usize;
            dbg!(index);
            count[index] += 1;
        } // end of this loop will give us a count of each character in the string

        dbg!(&count);

        let mut key = String::from("");
        dbg!(key.clone());

        for k in count.iter() {
            key += "#";
            key += k.to_string().as_str();
        } // end of this loop will form a key representing the frequency of each character in the string
        dbg!(key.clone());
        // combine similar titles (have the same frequency of each alphabet) into a group
        res.entry(key).or_insert(Vec::new()).push(s);
        dbg!(&res);

        count = vec![0; 26]; // reset count for next string
    }

    res.into_values().collect()
}

fn main() {
    // Sample tiles input
    let titles = vec![
        "duel".to_string(),
        "dule".into(),
        "speed".into(),
        "spede".into(),
        "deul".into(),
        "cars".into(),
    ];
    let output: Vec<Vec<String>> = group_titles(titles);
    dbg!(&output);
    let query = String::from("dule");

    for o in output.into_iter() {
        if o.contains(&query) {
            println!("{:?}", o);
        }
    }
}
