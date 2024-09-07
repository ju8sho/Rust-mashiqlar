fn main() {
/*№ 1
Butun sonlar massivi berilgan. Konsolga faqat 5ga bo'linadigan raqamlarni chop eting. */
// Butun sonlar massivini yaratamiz
let numbers = [10, 15, 23, 40, 56, 70, 80, 91, 100];

let res: Vec<_> = numbers.iter().filter(|&&i| i % 5 == 0).collect();

println!("massivni 5ga bolinadigan qiymatlari:{:?}", res);

/*№ 2
Chiziqni hisobga olgan holda: Ushbu qatorning dastlabki uchta belgisini oling:*/
let txt = "abcdef";

let uchta: String = txt.chars().take(3).collect();
println!("birinchi 3ta belgi: {:?}", uchta);

/*№ 3

Berilgan belgi:Ushbu belgi katta yoki kichik harf bilan yozilganligini bilib oling.*/
let chr: char = 'a';

if chr.is_lowercase() {
    println!("kichik")
}else {
    println!("katta")
}

/*№ 4
Massiv berilgan: Undan quyidagi bo'lakni [1, 2, 3] oling*/
let arr = [1, 2, 3, 4, 5, 6];

let arr_uch = &arr[0..3];
println!("massivni birinchi uchta qiymati:{:?}", arr_uch);

/*№ 5
Butun sonlar massivi berilgan: Birinchi nolning o'rnini konsolga chop eting.*/
let arr = [1, 2, 3, 0, 4, 5];

for i in 0..arr.len() {
    if arr[i] == 0 {
        println!("Array {:?}",i);
        break;
    }
}



}
