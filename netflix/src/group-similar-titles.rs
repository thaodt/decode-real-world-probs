use std::collections::HashMap;

// reference: https://tfpk.github.io/lifetimekata/chapter_3.html#example-2-only-one-reference-in-the-input-1
#[allow(clippy::needless_lifetimes)]
fn group_titles<'a>(strs: Vec<&'a str>) -> Vec<Vec<&'a str>> {
    let mut res: HashMap<String, Vec<&str>> = HashMap::new();
    let mut count = vec![0; 26]; // 26 English alphabets

    for s in strs.into_iter() {
        for c in s.chars() {
            let index = (c as u32 - 'a' as u32) as usize;
            dbg!(index);
            count[index] += 1;
        } // end of this loop will give us a count of each character in the string

        // dbg!(&count);

        let mut key = String::from("");
        // dbg!(key.clone());

        for k in count.iter() {
            key += "#";
            key += k.to_string().as_str();
        } // end of this loop will form a key representing the frequency of each character in the string
          // dbg!(key.clone());
          // combine similar titles (have the same frequency of each alphabet) into a group
        res.entry(key).or_insert(Vec::new()).push(s);
        dbg!(&res);

        count = vec![0; 26]; // reset count for next string
    }

    print_type_of(&res);

    res.into_values().collect()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // Sample tiles input
    let titles = vec!["duel", "dule", "speed", "spede", "deul", "cars"];
    let output: Vec<Vec<&str>> = group_titles(titles);
    dbg!(&output);

    let query = "dule";

    for o in output.into_iter() {
        if o.contains(&query) {
            println!("{:?}", o);
        }
    }
}
