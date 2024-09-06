fn main() {
/*№ 1
100 dan 1 gacha bolgan sonlarni konsolga chiqaring */
for i in (1..=100).rev() {
    println!("{}", i);
}

/*№ 2
Ba'zi qator berilgan: Satr oxirid boshlab barcha belgilarni birma-bir takrorlang va konsolga chiqaring.*/
let txt = "abcde";
for char in txt.chars().rev() {
    println!("{}", char);
}

/*№ 3
Baytlar sonini o'z ichiga olgan butun son berilgan: Ushbu qiymatni gigabayt, megabayt va kilobaytga aylantiring.*/
let b: i64 = 7435421243;

let gb = b as f64 / 1024.0 / 1024.0 / 1024.0;
let mg = b as f64 / 1024.0 / 1024.0;
let kb = b as f64 / 1024.0;

println!("{:.2} GB, {:.2} MG, {:.2} KB", gb, mg, kb);

/*№ 4
Shu sonning raqamlari yig‘indisini toping. */
let num: i32 = 1234567;
let sum: i32 = num
    .to_string() //string ga o'tgazamiz
    .chars() //stringni char tipiga aylantiramiz
    .map(|c| c.to_digit(10).unwrap() as i32) // map() har bir charni olib 10 lik sanoq tizimiga o'tkazamiz
    .sum(); // har bir olingan sonlarni yigindisini olamiz
println!("Yig'indi: {}", sum);

/*№ 5
Bu raqamning 10foizini toping . */
let num: f32 = 12.3;
let topish = num * 0.10;
println!("{} ning 10 foizi {}", num, topish);

}
