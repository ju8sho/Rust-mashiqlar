fn main() {
/* № 1
i8 Turi va u8 turi o'rtasidagi farq nima ekanligini ayting.*/
//javob: i8 turdagi malumotlar tuzulmasi butun va manfiy sonlarni qabul qiladi, u8 esa faqat butun sonni qabul qiladi

/*№ 2
Butun son berilgan: Ushbu raqamning oxirgi raqamini konsolga chop eting.*/
let num: i32 = 3446;
let oxirgi_raqam = num % 10; //xar qanday sonni 10 bolganda qoldig'i shu sonning oxirgi raqamoga teng boladi (%) operatori foyizini chiqaradi
println!("oxirgi raqam: {}", oxirgi_raqam);

/*№ 3
Butun son berilgan: Ushbu raqamning birinchi raqamini konsolga chop eting.*/
let mut num: i32 = 123;   // i32 tipda bolgani uchun biz -123 ni ham tekshirishimiz kerak
if num < 0 {   //agar son manfiy bolsa musbat qilib olamiz agar son musbat bolsa shart ozgarmaydi
    num = -num;   // -(-123) => 123 degani yani soni musbatga aylantiramiz
}
while num >= 10 { //toki num 10 dan katta bolsa 
    num /= 10; // har safar num ni 10 ga bolamiz va qoldiq natijani numga teglaymiz
   
}
println!("birinchi raqam: {}", num);

/*№ 4
Butun son berilgan: Ushbu raqamning birinchi va oxirgi raqamlari yig'indisini konsolga chop eting.*/
let mut num: i32 = 123;

let oxirgi_raqam = num % 10; //oxirgi raqamni olish

if num < 0 {num = -num;} // agar berilgan son manfiy bolsa musbat qilib olamiz va

while num >= 10 { //berilgan qiymatni 1- raqamini olamiz
    num /= 10; //bolingan sonni qoldiqni num ozgaruvchiga tenglaymiz 
}
let birinchi_raqam = num;
let sum = oxirgi_raqam + birinchi_raqam;
println!("birinchi va oxirgi raqamlar yigindisi: {}", sum);

/*№ 5
Butun son berilgan: Ushbu raqamdagi raqamlar sonini chop eting.*/
let mut num: i32 = 12345;

if num < 0 {num = -num} // manfiy sonni tekshirib olamiz va musbatga aylantiramiz

let mut sanoq = 0;  // raqam sonini hisoblash uchun sanog'ich

if num == 0 {
    sanoq = 1;
}else {
    while num > 0 {  
        num /= 10;
        sanoq += 1;
    }
}
println!("Raqamlar soni {}", sanoq); 

/*№ 6
Butun son berilgan: Sonning juft yoki toqligini tekshirish.*/
let num: i32 = 1234;
if num % 2 ==0 {
    println!("{} soni juft son", num);
}else {
    println!("toq son");
}

/*№ 7
Berilgan raqam: Bu raqam ikki xonali ekanligini tekshiring.*/
let num: u8 = 10;
if num >= 10 && num < 100 {
    println!("berilgan son:{} ikki xonalik",num);
}else {
    println!("raqam ikki xonalik emas")
}

}