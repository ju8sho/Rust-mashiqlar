fn main() {
/*№ 1
loop tsikl va while tsikl o'rtasidagi farqni tushuntiring. JAVOB:
Cheksiz sikl: loop sikli cheksiz takrorlanadigan sikl bo'lib, to'xtatish sharti aniq ko'rsatilmagan bo'lsa, u doimiy ravishda bajariladi.
Shartli sikl: while sikli berilgan shart bajarilgunga qadar takrorlanadi. Agar shart noto'g'ri bo'lsa, sikl hech qachon ishlamaydi. */
let mut son = 0;

loop {
    son += 1;
    println!("loop skli:{}", son);
    if son == 5 {
        break;
    }
}

let mut num = 0;

while num != 5 {
    num += 1;
    println!("while skli: {}",num);
}

/*№ 2
Butun sonlar massivi berilgan: Ushbu massivdagi elementlar sonini konsolga chop eting.*/
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let mass_soni = arr.len();
println!("elementlar soni: {} ta", mass_soni);

/*№ 3
Butun sonlar massivi berilgan: Ushbu massivning elementlarini konsolga chop eting.*/
let arr: [i32; 5] = [1, 2, 3, 4, 5];
println!("array elementlari: {:?}", arr);

/*№ 4
Satrlar bilan massiv berilgan: Ushbu massivning har bir elementining birinchi belgilarini konsolga chop eting.*/
let arr: [&str; 3] = ["123", "456", "789"];
for i in arr.iter() {
    if let Some(birinchi_belgi) = i.chars().next() {
        println!("{}",birinchi_belgi);
    }
}

/*№ 5
Ba'zi bir butun son berilgan: Uning barcha juft raqamlari yig‘indisini toping.*/
let num: u16 = 12345;

let yigindi: u16 = num
    .to_string()
    .chars()
    .filter_map(|x| x.to_digit(10))
    .filter(|&y| y % 2 == 0)
    .map(|r| r as u16)
    .sum();
println!("Juft raqamlar yig'indisi:{}", yigindi);
/*№ 6
Bu sonning faktorialini toping. */
let num: u8 = 12;
let mut faktorial: u128 = 1;
for i in 1..=num {
    faktorial *= i as u128;
}
println!("{} sonini faktoriali {}", num, faktorial);

}
