fn main() {
/*№ 1
Raqamlar qatori berilgan: Ushbu massivning noldan katta va o‘ndan kichik bo‘lgan elementlari yig‘indisini toping.*/
let arr: [i8; 6] = [-1, 2, -3, 4, 5, 11];
let katt_kichik_yigindi: i8 = arr
    .iter()
    .filter(|&&x| x > 0 && x < 10)
    .sum();
println!("katta va kichik massiv yigindisi:{:?}", katt_kichik_yigindi);

/*№ 2
Butun sonlar massivi berilgan: Ushbu massivning juft sonli elementlarini konsolga chop eting.*/
let arr: [u8; 5] = [1, 2, 3, 4, 5];
let juft_son: Vec<_> = arr
    .iter()
    .filter(|&&x| x % 2 == 0)
    .collect();
println!("massiv juft sonlari:{:?}", juft_son);

/* № 3
Butun sonlar massivi berilgan:Ushbu massivning elementlarini birinchi nolga qadar konsolga chop eting.*/
let arr: [u8; 6] = [1, 2, 3, 0, 4, 5];
for &i in arr.iter() {
    if i == 0 {
        break;
    }
    println!("{:?}", i);
}

}
