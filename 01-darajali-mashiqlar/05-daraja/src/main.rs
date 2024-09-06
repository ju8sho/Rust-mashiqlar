fn main() {
/*№ 1
i16 turi qancha va qanday qiymatlarni qabul qilishi mumkinligini bizga ayting.
Javob: i16 manfiy va musbat sonlarni -128 dan manfiy 127 gacha sonlarni qabul qiladi*/

/*№ 2
-100 dan 100 gacha bo'lgan oraliqda 7 ga bo'linadigan barcha sonlarni konsolga chop etish.*/
for i in -100..=100 {
    if i % 7 == 0 {
        println!("{}", i);
    }
}

/*№ 3
Ikkita butun son berilgan: Ushbu raqamlarning birinchi raqamlari mos kelishini tekshiring.*/
let num1: u16 = 12;
let num2: u16 = 13;

let son1 = num1.to_string().chars().next().unwrap();
let son2 = num2.to_string().chars().next().unwrap();

if son1 == son2 {
    println!("sonlar bir biriga teng");
}else {
    println!("sonlar bir biriga mos emas");
}

/*№ 4
Ikkita butun son berilgan: Birinchi raqam ikkinchisiga qoldiqsiz bo'linishini tekshiring.*/
let num1: u16 = 36;
let num2: u16 = 12;

if num1 % num2 == 0 {
    println!("{} qoldiqsiz bolinadi {} ga", num1, num2);
}else {
    println!("{} qoldiqli bolinadi {} ga", num1, num2);
}
 /*№ 5
 Satrdagi barcha belgilarni birma bir oxiridan boshlab chiqarish. */
 let txt: &str = "example";
  for i in txt.chars().rev() {
    println!("{}", i);
  }


/*№ 6
Uchta tamsayı berilgan: Konsolga ushbu raqamlarning kattaroq qismini chop eting.*/
let num1: u16 = 36;
let num2: u16 = 24;
let num3: u16 = 12;

let max_num = num1.max(num2).max(num3);
println!("{}", max_num);

/*№ 7
Ushbu belgilarni qatorga birlashtiring: */
let chr1: char = 'a';
let chr2: char = 'b';
let chr3: char = 'c';

// chr1.to_string();
// chr2.to_string();
// chr3.to_string();
let mut txt = String::new();
txt.push(chr1);
txt.push(chr2);
txt.push(chr3);

println!("{}", txt);


/*№ 8
Kilobaytlar sonini o'z ichiga olgan butun son berilgan: Ushbu qiymatni baytga aylantiring.*/
let kb:u32 = 35;

let bayt = kb * 1024;
println!("{} kilobayt {} baytga teng", kb, bayt);


/*№ 9
1dan 31gacha bo'lgan kun sonini o'z ichiga olgan butun son berilgan: Ushbu kun oyning qaysi o'n kunligiga to'g'ri kelishini aniqlang.*/
let num: u16 = 1;
let kunlik = match num {
    1..=10 => "Birinchi o'n kunlik",
    11..=20 => "Ikkinchi o'n kunlik",
    21..=31 => "Uchinchi o'n kunlik",
    _ => "Kun sanasi notug'ri kiritilgan"
};
println!("berilgan {}-kun oyning {} ga to'g'ri keladi", num, kunlik);

}
