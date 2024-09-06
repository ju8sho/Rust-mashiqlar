fn main() {
/*№ 1
Butun sonlar massivi berilgan: Ushbu massiv elementlarining kvadratlari yig‘indisini toping.*/
let arr: [i32; 5] = [1, 2, 3, 4, 5];

let arr_yigindi: i32 = arr
    .iter()
    .map(|&x| x * x)
    .sum();
println!("kvadrat yigindisi: {}", arr_yigindi);

/*№ 2
Butun sonlar massivi berilgan: Bu massivning o‘rta arifmetik qiymatini toping.*/
let arr = [1, 2, 3, 4, 5];

let qiymat = arr.iter().sum::<i32>();
let orta_arfmetik_qiymat = qiymat as f32 / arr.len() as f32;
println!("orta arfmetik qiymat:{}", orta_arfmetik_qiymat);

/*№ 3
Ba'zi bir butun son berilgan: Uning birinchi uchta raqamini konsolga chop eting.*/
let num: u16 = 12345;
let str_num: String = num.to_string();
let birinchi_uchta_raqam: String = str_num.chars().take(3).collect();
println!("Birinchi uchta raqam: {}", birinchi_uchta_raqam);



/*№ 4
Massiv berilgan: Ushbu massivni 1dan 10gacha butun sonlar bilan to'ldirish uchun tsikldan foydalaning.*/
let mut arr: [u8; 10] = [0; 10];

for i in 0..arr.len() {
    arr[i] = (i + 1) as u8;
}
println!("arraylar {:?}", arr);
}
