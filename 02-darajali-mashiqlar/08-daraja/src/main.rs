fn main() {
/*№ 1
Satrlar bilan massiv berilgan: Ushbu satrlarning belgilar qatorini oling:['a', 'b', 'c', 'd', 'e', 'f']*/
let arr: [&str; 3] = ["ab", "cd", "ef"];
let chars: Vec<char> = arr
    .iter()
    .flat_map(|&x| x.chars())
    .collect();
println!("massiv char:{:?}", chars);

/*№ 2
1dan 12gacha bo'lgan oy sonini o'z ichiga olgan butun son berilgan: Ushbu raqamga mos keladigan oy nomini chop eting.*/
let num: u8 = 1;
let oy = match num {
    1 => "Yanvar",
    2 => "Fevral",
    3 => "Mart",
    4 => "Aprel",
    5 => "May",
    6 => "Iyyun",
    7 => "Iyyil",
    8 => "Avgust",
    9 => "Sentyabr",
    10 => "Oktyabr",
    11 => "Noyabr",
    12 => "Dekabr",
    _ => "Berilgan oy soni notug'ri"
};
println!("berilgan oy soni:{} - {} oyiga togri keladi", num, oy);

/*№ 3
Chiziqni hisobga olgan holda: Ushbu satrning birinchi va oxirgi belgisini quyidagicha oling:"15"*/
let txt: &str = "12345";
let mut yigindi = String::new();
let birinchi = txt.chars().next().unwrap();
let oxirgi = txt.chars().last().unwrap();
yigindi.push(birinchi);
yigindi.push(oxirgi);
println!("birinchi va oxirgi {}", yigindi);

/*№ 4
To'rtburchakning tomonlarini o'z ichiga olgan o'zgaruvchilar berilgan: Ushbu to'rtburchakning maydoni va perimetrini oling.*/
let a: i16 = 10;
let b: i16 = 20;

let maydon = a * b;
let parametr = 2 * (a + b);
println!("to'rtburchakning maydoni:{} \nto'rtburchakning perimetrini:{}", maydon, parametr);

}
