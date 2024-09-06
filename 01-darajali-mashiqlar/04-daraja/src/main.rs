fn main() {
/*№ 1
i8 Turi qancha va qanday qiymatlarni qabul qilishi mumkin?
Javob: i8 turi -128 dan musbat 127 gacha sonlarni qabul qiladi */

/*№ 2
i16 Tur va f32 tur o'rtasidagi farq nimada?. Javob : i16 manfiy va musbat sonlarni qabul qiladi, f32 turi faqat float sonlari qabul qiladi 1.0, 1.4, 23.4 */

/*№ 3
2dan 100 gacha bo'lgan oraliqdagi barcha juft raqamlarni konsolga chop eting.*/
for i in 2..=100 {
    if i % 2 == 0 {
        println!("{}", i);
    }
}

/*№ 4
Ba'zi qator berilgan: Takrorlang va uning barcha belgilarini birma-bir konsolga chiqaring.*/
let txt: &str = "abcde";
for i in txt.chars() {
    println!("{}", i);
}

/*№ 5
Ikkita butun son berilgan: Birinchi raqamni ikkinchisiga bo'lishda qoldiqni oling.*/
let num1: u16 = 36;
let num2: u16 = 12;

let qoldiq = num1 % num2;
println!("{}", qoldiq);

/*№ 6
0dan 60 gacha bo'lgan daqiqa raqamini o'z ichiga olgan butun son berilgan: Bu qiymat soatning qaysi choragiga to'g'ri kelishini aniqlang.*/
let num: u8 = 30;

let chirak = match num {
    0..=14 => "1-chorak",
    15..=29 => "2-chorak",
    30..=44 => "3-chorak",
    45..=59 => "4-chorak",
    _ => "Notug'ri raqam"
};
println!("{} daqiqa soatning {}",num, chirak);

/*№ 7
Ikki xonali raqam berilgan: Ikkinchi raqam birinchisidan kattaroq ekanligini tekshiring.*/
let mut num: u8 = 12;

let ikkinchi = num % 10;

while num >= 10 {
    num /= 10;
}
let birinchi = num;

if ikkinchi > birinchi {
    println!("ikkinchi raqam katta")
}


}
