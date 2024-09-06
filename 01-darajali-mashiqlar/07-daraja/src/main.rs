fn main() {
/*№ 1
u16 Turi qancha va qanday qiymatlarni qabul qilishi mumkin.Javab u16 turi faqat musbat sonlar 65535 gacha son qabul qiladi*/

/*№ 2
1dan 100gacha bo'lgan oraliqdagi barcha toq sonlar kvadratlari yig'indisini toping. */

let sum: i32 = (1..=100) //oraliq yaratish
    .filter(|&x| x & 2 != 0) //toq sonlarni olish
    .map(|x| x * x) //kvadratni hisoblash
    .sum(); // yigindini olish
    println!("1dan 100gacha bo'lgan oraliqdagi barcha toq sonlar kvadratlari yig'indisi {}", sum);

/*№ 3
N darajada ikkilik ko'paytmalarini hisoblaydigan kod yozish. */
let n = 10; // n ning darajasi 10 ga teng

for i in 0..n {
    let qadam = 2_u32.pow(i); // 2 sonini i darajasi
    println!("2^{} = {}", i, qadam);
}


/*№ 4
Uch qator berilgan: Ushbu satrlarning qiymatlarini butun sonlar sifatida qo'shing.*/
let txt1: &str = "123";
let txt2: &str = "456";
let txt3: &str = "789";

let num1: i32 = txt1.parse().expect("stringni songa aylantirishda xatolik");
let num2: i32 = txt2.parse().expect("stringni songa aylantirishda xatolik");
let num3: i32 = txt3.parse().expect("stringni songa aylantirishda xatolik");

let qiymat = num1 + num2 + num3;
println!("qoshilgani qiymatlar: {}", qiymat);

/*№ 5
Ba'zi qator berilgan: Shu qatordagi raqamlar yig‘indisini toping.*/
let txt: &str = "123456789";
let yigindi: u32 = txt.chars()
    .filter_map(|i| i.to_digit(10))// Har bir belgi raqam ekanligini tekshirish va ularni raqamga aylantirish
    .sum();
println!("yig'indi {}", yigindi);

/*№ 6
Gigabayt sonini o'z ichiga olgan kasr raqam berilgan: Ushbu qiymatni megabayt, kilobayt va baytga aylantiring.*/
let gb: f32 = 35.24;
let mg = gb * 1024.0;
let kb = mg * 1024.0;
let bayt = kb * 1024.0;

println!("mg:{} kb:{} bayt:{}", mg, kb, bayt);




}
