fn main() {
/*№ 1
i8 Turi necha baytni egallaydi. i8 turi (1bayt) egalaydi*/

/*№ 2
1dan 10 gacha bolgan butun sonlarni chop eting.*/
for i in 1..=5 {
    println!("{}", i)
}

/*№ 3
Bir qator berilgan. Agar ushbu satrda bir nechta belgilar mavjud bo'lsa, ushbu satrning oxirgi belgisini konsolga chop eting. */
let matn = "Salom Rust".to_string();
if matn.len() > 0 {
    let char = matn.chars().last().unwrap();
    println!("oxirgi harif:{}", char);
}else {
    println!("matin bo'sh");
}
// ohirgidan 2-harfni chiqaish
if matn.len() > 0 {
    let char: Vec<char> = matn.chars().collect();
    let oxirgidan_oldingi = char[char.len() - 2]; // charni qiymatidan 2 ni ayiramiz 
    println!("oxirgidan oldingi harf:{}", oxirgidan_oldingi);
}else {
    println!("matn bosh");
}

/*№ 4
Butun son berilgan. Ushbu raqamning birinchi va oxirgi raqamlari mos kelishini tekshiring. */
let mut son = 12344;

let oxirgi_son = son % 10; //oxirgi sonni olish

while son >=10 { // birinchi sonni olish
    son /= 10;
}

let birinchi_son = son;

if birinchi_son == oxirgi_son {
    println!("birinchi son:{} oxirgi son:{} ga mos keladi", birinchi_son, oxirgi_son);
}else {
    println!("birinchi son oxirgi songa mos emas")
}


/*№ 5
Ikkita butun son berilgan: Konsolga ushbu raqamlarning kattaroq qismini chop eting.*/
let num1: u16 = 36;
let num2: u16 = 12;

let kattaroq_son = if num1 > num2 {num1} else {num2};
println!("Berilgan raqamlardan kattarog'i: {}", kattaroq_son); 


/*№ 6
Butun sondan iborat qator berilgan: Ushbu qatorni raqamga aylantiring:*/
let txt: &str = "123";
match txt.trim().parse::<i32>() {
    Ok(son) => println!("satir songa aylantirildi:{}", son),
    Err(_) => println!("xatolik bor"),
}

/*berilgan butun son o'zi bilan oyning raqamini ifodalaydi (1 dan 12 gacha). Sizdan ushbu oyni qaysi faslga tegishli ekanligini aniqlash so'ralmoqda. */
let num: u8 = 1; // misol 1- oy 2- oy 

let fasl = match num {
    12 | 1 | 2 => "Qish",
    3 | 4 | 5 => "Bahor",
    6 | 7 | 8 => "Yoz",
    9 | 10 | 11 => "Kuz",
    _ => "Notugri oy raqami"
};
println!("Bu oy {} fasliga tegishli", fasl)


}
