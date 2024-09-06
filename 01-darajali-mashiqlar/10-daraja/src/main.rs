fn main() {
/*№ 1
1dan 100gacha bo'lgan oraliqdagi barcha juft butun sonlar yig'indisini toping. */
for i in 1..=100 {
    if i % 2 == 0 {
        println!("{}", i)
    }
}

/*№ 2
Bir yilda necha soat borligini aniqlang. 
1 kunda 24 soat
1 yil 365 kun bor
365 * 24 */
let bir_kun_soati = 24;
let bir_yil_kuni = 365;
let barcha_soatlar = bir_yil_kuni * bir_kun_soati;
println!("1yilda {} soat bor", barcha_soatlar);

/*№ 3
Raqam berilgan: Uni aylantiring:*/
let num: u32 = 1234567;

let num_str = num.to_string();
let rev_str_num: String = num_str.chars().rev().collect();

let rev_num: u32 = rev_str_num.parse().unwrap();
println!("reverse son {}", rev_num);


/*№ 4
Ba'zi bir butun son berilgan: Ushbu raqamning barcha bo'luvchilarini konsolga chop eting.*/
let num: u16 = 18;

println!("{} soninig bo'linuvchilari:", num);
for i in 1..num {
    if num % i == 0 {
        println!("{}",i);
    }
}

/*№ 5
Raqam berilgan: Ushbu sonni 2 ga bo'ling, natija 10 dan kichik bo'lguncha davom eting. Buning uchun qancha marta bo'lish kerakligini aniqlang.*/
let mut son: i32 = 1000;

let mut sanoq = 0;
while son >= 10 {
    son /= 2;
    sanoq += 1;
}
println!("Sonni 2 ga bo'linganda natija 10 dan kichik bo'lishi uchun {} marta bo'linadi", sanoq);

/*№ 6
10 ning darajasi sifatida berilgan bir son (masalan, 1000) berilgan. bu son 10 ning qaysi darajasi ekanligini aniqlang*/
let num: u16 = 1000;
let mut temp = num;
let mut count = 0;

while temp > 1 {
    temp /= 10;
    count += 1;
}
println!("{} soni 10 ning {}-darajasi", num, count)

}
