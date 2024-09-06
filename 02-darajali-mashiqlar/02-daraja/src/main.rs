fn main() {
/*№ 1
1dan 100gacha bo‘lgan barcha butun sonlarning o‘rta arifmetik qiymatini toping. */
let yigindi: f32 = (1..=100).sum::<i32>() as f32;
let orta_arfmetik_qiymat = yigindi / 100.0;
println!("1dan 100 gacha bolgan sonlarning orta arifmetik yigindisi: {}", orta_arfmetik_qiymat);

/*№ 2
Butun sonlar massivi berilgan: Ushbu massiv egallagan baytlar sonini konsolga chop eting.*/
let arr: [i8; 5] = [1, 2, 3, 4, 5];
let element_hajmi = std::mem::size_of_val(&arr);
println!("Massiv egallagan bayitla soni:{}", element_hajmi);

/*№ 3
Berilgan raqam: Shu sonning birinchi va oxirgi raqamlari yig‘indisini toping.*/
let num: u16 = 12345;

let raqamlar: Vec<_> = num.to_string().chars().collect();

let birinchi = raqamlar.first()
    .and_then(|x| x.to_digit(10))
    .unwrap_or(0);

let oxirgi = raqamlar.last()
    .and_then(|x| x.to_digit(10))
    .unwrap_or(0);

let yigindi = birinchi + oxirgi;
println!("yig'indi: {}", yigindi);

/*№ 4
Butun sonlar massivi berilgan: Ushbu massiv elementlarining yig‘indisini toping.*/
let arr = [1, 2, 3, 4, 5];

let yigindi: i32 = arr.iter().sum();
println!("array yigindisi: {}", yigindi);

/*№ 5
Ba'zi bir butun son berilgan: Uning barcha raqamlarini oxiridan konsolgacha chop eting.*/
let num: u16 = 12345;
let teskari: String = num
    .to_string()
    .chars()
    .rev()
    .collect();
println!("teskari raqam: {}",teskari)

}
