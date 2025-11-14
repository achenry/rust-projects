fn main() {
    let x = 2.0;
    let y: f32 = 3.0;
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let trunc = -5 / 3;
    let rem = 43 % 3;
    
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;

    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}
