use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    println!("Vector[2]: {}", v.get(2).unwrap());

    println!("Vector: {v:?}");

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert("Yellow".to_string(), 50);

    println!("Blue's score: {}", scores.get("Blue").unwrap());
    println!("Yellow's score: {}", scores.get("Yellow").unwrap());

    scores.keys().for_each(|k| println!("{k}'s score: {}", scores.get(k).unwrap()));
}
