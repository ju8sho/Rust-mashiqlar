

fn main() {
/*№ 1
Butun sonlar massivi berilgan: Ushbu massiv elementlarining kvadratlari yig‘indisini toping.*/
let arr: [u8; 5] = [1, 2, 3, 4, 5];
let kv_yigindi: u32 = arr.iter().map(|&x| x as u32 * x as u32).sum();
println!("massiv kvadrat yigindisi:{:?}", kv_yigindi);

/*№ 2
Raqamlar qatori berilgan: Ushbu massivning elementlarini satrga birlashtiring:*/
let arr: [u8; 5] = [1, 2, 3, 4, 5];
let satr: String = arr
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>().join(" ");
println!("massiv satirga aylandi:{}", satr);

/*№ 3
Sana bilan massiv berilgan: Ushbu massivning elementlaridan quyidagi formatda sana to'plang: "31-12-2025"*/
let arr: [&str; 3] = ["2025", "12", "31"];
let sana = format!("{}-{}-{}", arr[2], arr[1], arr[0]);
println!("sana:{}", sana);

}
