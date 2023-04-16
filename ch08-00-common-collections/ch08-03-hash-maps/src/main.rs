use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 型注釈が必要だけど、アンダースコアを使用して、ベクタのデータ型に基づいて型推論することができる
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // ここではもうすでに所有権はHashMapに移っている
    // println!("{}", field_name)

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(&score) => score,
        None => {
            println!("{} is not found", team_name);
            &0
        },
    };
    println!("{:?}", score);

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50); // entryで存在チェックし、存在しない場合に値を挿入する
    println!("{:#?}", scores2)

}
