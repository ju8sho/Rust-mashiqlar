fn main() {
/*№ 1
Ba'zi bir butun son berilgan: Uning oxirgi uchta raqamini konsolga chop eting.*/
let num: u16 = 12345;
let str_num = num.to_string();
let teskari_uchta_raqam: String = str_num.chars().rev().take(3).collect();
let ucht_raqam: String = teskari_uchta_raqam.chars().rev().collect();

println!("oxirgi uchta raqam:{}", ucht_raqam);

/*№ 2
Raqamlar qatori berilgan:
Ushbu massiv elementlarining kvadrat ildizlari yig‘indisini toping. */
let arr = [1, 2, 3, 4, 5];
let ildiz_sum: f64 = arr.iter()
    .map(|&x| (x as f64).sqrt())
    .sum();
println!("Kvadrat ildizlarining yig'indisi: {}", ildiz_sum);

/*№ 3
Raqamlar qatori berilgan: Ushbu massivning har bir elementini ikki marta oshiring:*/
let arr = [1, 2, 3, 4];

let oshir: Vec<i32> = arr.iter().map(|&x| {
    if x % 2 == 0 {
        x * 2 // juft sonlarni 2 ga kopaytiramiz
    }else {
        x + x // toq sonlarni o'ziga qoshamiz
    }
}).collect();

println!("oshirilgan massiv:{:?}", oshir);

/*№ 4
Massiv berilgan: Ushbu massivni 10dan 1gacha butun sonlar bilan to'ldiring */
let mut arr: [u8; 10] = [0; 10];
for i in 0..10 {
    arr[i] = (10 - i) as u8
}
println!("massiv toldirilgan:{:?}", arr);

}