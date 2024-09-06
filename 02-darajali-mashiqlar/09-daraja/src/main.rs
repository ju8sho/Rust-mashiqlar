fn main() {
/*№ 1
Massivni 100dan 1gacha bo'lgan juft raqamlar bilan to'ldirish uchun tsikldan foydalaning. */
let mut arr: [i32; 50] = [0; 50];
let mut num = 100;
for i in 1..50 {
    arr[i] = num;
    num -= 2;
}
println!("toldirilgan juft arrlar:{:?}", arr);

/*№ 3
1dan 7gacha bo'lgan hafta kunining sonini o'z ichiga olgan butun son berilgan: Ushbu o'zgaruvchida hafta oxiri yoki ish kuni borligini aniqlang.*/
let num: u8 = 1;
let res = match num {
    1..=5 => "Ish kuni",
    6..=7 => "Hafta oxri dam olish kunlari",
    _ => "Notugri kun son berilgan",
};
println!("Berilgan kun:{} - {}",num, res);

/*№ 4
selsiy bo'yicha haroratni o'z ichiga olgan o'zgaruvchanlik berilgan: Bu haroratni Farengeyt gradusiga aylantiruvchi dastur yozing.*/
let tc: i16 = 25;
let tf = (tc as f32 * 9.0 / 5.0) + 32.0;
println!("{}°C -> Farangeyt bo'yicha: {:.1}°F teng", tc, tf);

}
