fn main() {
/*№ 1
Raqamlar qatori berilgan: Bu massivning musbat elementlari yig‘indisini toping.*/
let arr: [i8; 5] = [1, 2, -3, 4, -5];
let musbat_yigindi: i8 = arr.iter().filter(|&&x| x > 0).sum();
println!("Musbat elementlar yig'indisi: {:?}", musbat_yigindi);

/*№ 2
Raqamlar qatori berilgan: Ushbu massivning elementlarini konsolga teskari tartibda chop eting.*/
let arr: [u8; 5] = [1, 2, 3, 4, 5];
let teskari: Vec<_> = arr.iter().rev().collect();
println!("teskari massiv:{:?}", teskari);

/*№ 3
Butun sonlarni o'z ichiga olgan qatorlar berilgan: Ushbu massivni yangi massivning qiymatlari butun songa aylanishi uchun aylantiring:*/
let arr: [&str; 3] = ["123", "456", "789"];
let musbat: Vec<i16> = arr
    .iter()
    .map(|x| x.parse::<i16>().unwrap())
    .collect();

println!("musbat sonlar:{:?}", musbat);

}
