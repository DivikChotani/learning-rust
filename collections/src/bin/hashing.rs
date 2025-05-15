use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("blue".to_string(), 30);

    scores.insert("yellow".to_string(), 50);

    let blue_score = match scores.get(& "blue".to_string()).copied(){
        Some(x) => x,
        None => 0
    };

    let m = String::from("magenta");
    let t = m.clone();
    scores.insert(m, 20);

    //println!("{m}"); can't access m anymore because you lose access

    //get takes a reference, so have to pass as a reference, and returns an option ref, so you have to copy (or clone if string)
    //and then unrwap or match

    println!("{blue_score}");

    println!("{scores:#?}");

    let mut m = scores.entry(t.clone()).or_insert(2); //gives default if doesnt exist
    //t will lose access if passed without copy
    *m = 100000; //gets reference


    m = scores.entry(t + "yellow").or_insert(2);

    println!("{scores:#?}")


    
}