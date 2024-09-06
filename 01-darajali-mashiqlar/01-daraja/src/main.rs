fn main() {
/* № 1
Berilgan raqam:Salbiy yoki yo'qligini tekshiring. Ushbu ma'lumotni konsolga chop eting.*/
let num: i32 = 123;
if num < 0 {
    println!("Salbiy")
}else {
    println!("Salbiy emas")
}

/*№ 2
Chiziqni hisobga olgan holda:Ushbu satrning uzunligini konsolga chop eting.*/
let txt: &str = "abcde";
println!("{}",txt.len());

/*№ 3
Chiziqni hisobga olgan holda:Konsolga satrning oxirgi belgisini chop eting. */
let txt: &str = "abcde";
if let Some(oxirgi) = txt.chars().last() {
    println!("{}- satirining oxirgi belgisi:{}", txt, oxirgi)
}else {
    println!("Satir bo'sh")
}

/*№ 4
Ikkita so'z beriladi.Ushbu so'zlarning birinchi harflari mos kelishini tekshiring. */
let word1: &str = "abc";
let word2: &str = "ade";
let harf1 = word1.chars().next();//sozlarning harbir harifini char tipiga o'tqazib next()yordamida birinchi charni olish
let harf2 = word2.chars().next();
if  harf1 == harf2 {
    println!("birinchi hariflari bir biriga mos keladi");
}else {
    println!("hariflar mos kelmaydi")
}

/*№ 5
So'z berilgan: Uning oxirgi harfini oling. Agar so'z yumshoq belgi bilan tugasa, siz oxirgidan oldingi harfni olasiz.*/
let word: &str = "день";
let oxirgi_harf = word.chars().last();
match oxirgi_harf {
    Some('ь') => {
        let oxirgidan_oldingi_harif = word.chars().rev().nth(1); //chars() sozlarni char tipiga o'tqazadi, rev() teskarisiga aylantiradi, nht() indeks orqali qiymarni oladi shunda oxirgidan 2- element olinmoqda 
        match oxirgidan_oldingi_harif {
            Some(harf) => println!("oxirgi harif yumshoq oxirgidan oldingi harif: {}", harf),
            None => println!("Satir bo'sh")
        }
    }
    Some(harf) => println!("oxirgi harf {}", harf),
    None => println!("Satir bo'sh"),
    
}

/*№ 6
Berilgan raqam:Ushbu raqamni qatorga aylantiring:*/
let num: u16 = 123;
let satr = num.to_string();
println!("{}", satr);

/*№ 7
Berilgan raqam: berilgan raqamdan 1 dan 10 gacha bolgan oraliqda ekanligini tekshirish*/
let num: i16 = 5;
if (1..=10).contains(&num) { // contains() berilgan 1..=10  oraliq ichida num uzgaruvchining qiymati bor yoki yoqligini tekshiradi shu oraliqga tushadimi yoki yoq
    println!("{} raqami 1 va 10 oralig'ida", num);
}else {
    println!(" berilgan raqam 1 va 10 oralig'da emas")
}

}
