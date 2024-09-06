fn main() {
/*№ 1
To'pning radiusini o'z ichiga olgan o'zgaruvchi berilgan: Ushbu sharning hajmi va sirtini toping.*/
let r: f32 = 10.3; //radius
let pi = std::f32::consts::PI;

// Hajm formulasi: V = 4/3 * π * r^3
let hajmi = (4.0 / 3.0) * pi * r.powi(3);

// Sirt formulasi: S = 4 * π * r^2
let sirt = 4.0 * pi * r.powi(2);

println!("sharning hajmi: {:.2} kub birligi", hajmi);
println!("Sharning sirt yuzasi: {:.2} kvadrat birligi", sirt);

/*№ 2
Ikkita butun son berilgan:Ushbu raqamlarning barcha umumiy bo'luvchilarini konsolga chop eting.*/
let num1: u16 = 12;
let num2: u16 = 16;

// Ikkala sonning bo'luvchilarini topish uchun funksiyani yaratamiz
let son1: Vec<u16> = (1..=num1).filter(|&i| num1 % i == 0).collect();
let son2: Vec<u16> = (1..=num2).filter(|&i| num2 % i == 0).collect();

// Umumiy bo'luvchilarni topamiz
let umumiy_bolinuvchi: Vec<u16> = son1
    .iter()
    .filter(|&&i| son2.contains(&i))
    .cloned()
    .collect();
println!("Umumiy bolinivchilar: {:?}", umumiy_bolinuvchi);

/*№ 3
Butun sonlar massivi berilgan: Ushbu massivning birinchi va oxirgi elementlarini almashtiring:;*/
let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

let oxirgi_element = arr.len() -1;
arr.swap(0, oxirgi_element);
println!("Almashtirilgan massiv:{:?}", arr);

/*№ 4
Bir qator 6ta raqamlar berilgan Birinchi uchta raqamning yig'indisi ikkinchi uchta raqamning yig'indisiga teng ekanligini tekshiring.*/
let num: u16 = 123321u32 as u16;
let str_num = num.to_string();

let birinchi_uchta: u32 = str_num[0..2].chars()
    .map(|i| i.to_digit(10).unwrap())
    .sum();

let ikkinchi_uchta: u32 = str_num[3..5].chars()
    .map(|i| i.to_digit(10).unwrap())
    .sum();

if birinchi_uchta == ikkinchi_uchta {
    println!("Yigindilar teng")
}else {
    println!("Yigindilar teng emas")
}





}
