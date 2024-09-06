fn main() {
/*№ 1
1dan 100gacha boʻlgan oraliqdagi barcha butun sonlarning kvadratlari yigʻindisini toping. */
let yigindi: i32 = (1..=100)
    .map(|x| x * x)
    .sum();
println!("1 dan 100 gacha bolgan butun sonlar kvadrat yigindisi: {}", yigindi);

/*№ 2
Ikkita so'z beriladi. Birinchi so'zning oxirgi harfi ikkinchi so'zning birinchi harfiga mos kelishini tekshiring. */
let txt1 = "Salom Rust";
let txt2 = "Hello Rust";

let oxirgi_harf = txt1.chars().last();
let birinchi_harf = txt2.chars().next();

if oxirgi_harf == birinchi_harf {
    println!("Ikki matinning oxirgi va birinchi harflari bir biriga mos keladi");
}else {
    println!("Ikki matinning oxirgi va birinchi harflari bir biriga mos kelmaydi");
}

/*№ 3
O'n kun ichida qancha soniya borligini aniqlang.
1 kun ichidagi soniyalarni hisoblash: 1 kun = 24 soat, 1 soat = 60 daqiqa, 1 daqiqa = 60 soniya.
Shunday qilib, 1 kun = 24 * 60 * 60 soniya.
10 kun ichidagi soniyalarni hisoblash:
10 kun = 10 * (1 kun ichidagi soniyalar soni). */
let kun_soni = 10;
let soniyalar_soni_bir_kun = 24 * 60 * 60;
let jami_soniyalar = kun_soni * soniyalar_soni_bir_kun;
println!("{} kun ichida {} soniya bor", kun_soni, jami_soniyalar);

/*№ 4
-100dan 100gacha barcha butun sonlarni chop eting. */
for i in -100..=100 {
    println!("{}", i)
}

/*№ 5
Beshta belgi berilgan: Ushbu belgilarni raqamga birlashtiring:*/
let chr1: char = '1';
let chr2: char = '2';
let chr3: char = '3';
let chr4: char = '4';
let chr5: char = '5';

let mut matn = String::new();
matn.push(chr1);
matn.push(chr2);
matn.push(chr3);
matn.push(chr4);
matn.push(chr5);

let son: i32 = matn.parse().unwrap();
println!("{}", son)

}
