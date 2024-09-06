fn main() {
/*№ 1
u8 Turi qancha va qanday qiymatlarni qabul qilishi mumkin.Javob: u8 tipi 0dan 255 gacha sonlar qabul qiladi*/

/*№ 2
1dan 100gacha bo'lgan barcha butun sonlar yig'indisini toping. */
let mut yigindi = 0;
for i in 1..=100 {
    yigindi += i;
}
println!("1dan 100gacha bo'lgan sonlar yig'indisi: {}", yigindi);

/*№ 3
Uchta belgi berilgan: Ushbu belgilarning qiymatlarini butun sonlar sifatida qo'shing.*/
let chr1: char = '1';
let chr2: char = '2';
let chr3: char = '3';

let son1 = chr1.to_digit(10).unwrap();
let son2 = chr2.to_digit(10).unwrap();
let son3 = chr3.to_digit(10).unwrap();

let yigindi = son1 + son2 + son3;
println!("char sonlarning yig'indisi: {}", yigindi);

/*№ 4
Megabayt sonini o'z ichiga olgan raqam berilgan: Ushbu qiymatni baytga aylantiring.*/
let mb: f32 = 35.5;

let bayt = mb * 1024.0 * 1024.0;
println!("{}mb {} baytga aylantirildi!", mb, bayt);

/*№ 5
Ikkita raqam berilgan: Birinchi raqam ikkinchidan necha foizini tashkil etadi.*/
let num1: f32 = 36.0;
let num2: f32 = 12.0;

let foyz = num1 / num2 * 100.0;
println!("{} soni {} soning {}% foyizini tashkil qiladi", num1, num2, foyz);


}
