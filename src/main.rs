#![allow(unused)]

use std::fs;
use std::env;

struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn add(&self) -> i32 {
        self.x + self.y
    }
}

struct User {
    id: i32,
    name: String,
    active: bool,
}

const ABC: i32 = 1;

fn build_user(template: User, id: i32, name: String) -> User {
    let user = User {
        id,
        name,
        ..template
    };
    return user;   
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("haha");
}

fn main7_f1(v: &[i32]) {
    println!("{:?} {} {:p} {:p}", v, v[0], &v[0], &v[1]);
}

fn main7_f2(s: &String) {
    println!("{:p}", s);
}

fn main7_f3(arr: &[i32]) {
    println!("{:p}", arr);

    println!("{}", usize::MAX);
}

fn main9_f1() {
    println!("--- main9_f1() ---");
    let content = fs::read_to_string("./Cargo.toml").unwrap();
    println!("{}", content)
}

fn main9_f2() {
    let port = env::var("PORT").unwrap_or("3001".to_string());
    println!("{}", port);    
}

fn main8_f2() {
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();

    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
        .iter()
        .inspect(|x| println!("The item is: {}", x))
        .map(|x| x * 2)
        .inspect(|mapped_x| println!("Then it is: {}", mapped_x))
        .collect::<Vec<i32>>();
}

fn main8_f1() {
    println!("--- Start main8 ---");
    let mut my_number = 9;
    my_number += 10;

    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>();    

    let mut my_number = dbg!(9);
    dbg!(my_number += 10);

    let new_vec = dbg!(vec![8, 9, 10]);

    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());

    dbg!(double_vec);    
}

fn main9() {
    main9_f1();
    main9_f2();
}

fn main8() {
    main8_f1();
    main8_f2();
}

fn main7() {
    println!("--- Start main7 ---");
    let s = String::from("hello");
    let slice2 = &s[0..0];

    let num1 = 1;
    let arr = [1,2];
    let num2 = 2;
    let v = vec![1,2];
    println!("{:?} {} {:p} {:p}", v, v[0], &v[0], &v[1]);
    println!("{:?} {} {:p} {:p} {:p} {:p}", arr, arr[0], &num1, &arr[0], &arr[1], &num2);

    let v = vec![1,2];
    main7_f1(&v);

    let s = String::from("hi");
    println!("{:p}", &s);
    main7_f2(&s);

    let arr = [3,4];
    main7_f3(&arr);
}

fn main6() {
    println!("--- Start main6 ---");
    let mut s = String::from("hello");
    change(&mut s);    
    println!("{}", s);

    let i:i32 = 0;
    let j = &i;
    let k = &j;
    let m = &&i;

    println!("{:p} {:p} {:p}", &i, &&i, &&&i);
}

fn main5() {
    println!("--- Start main5 ---");
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // ---

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);    
    
    // ---

    let r1 = String::from("abc");
    let r2 = &r1;
    println!("r1,r2: {},{}", r1, r2);

    // ---

    let r1 = "abc";
    let r2 = &r1;
    println!("r1,r2: {},{}", r1, r2);

    // ---

    let r1 = "abc"; // string literal can make copy
    let r2 = r1;
    println!("r1,r2: {},{}", r1, r2);

}

fn main3() {
    println!("--- Start main3 ---");
    let squares: Vec<_> = (0..10).map(|i| i * i).collect();
    println!("{:?}", squares);

    println!("{:?}", (0..10));

    let a = "abc";
    println!("{}", a);
}

fn main4() {
    println!("--- Start main4 ---");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    for c in "明天".chars() {
        println!("{}", c);
    }
    for b in "明天".bytes() {
        println!("{}", b);
    }    
    
    println!("서태지 size: {}", "서태지".bytes().len());
    println!("नमस्ते size: {}", "नमस्ते".bytes().len());
    println!("明天  size: {}", "明天".bytes().len());

    let s = String::from("明天明天明天明天明天明天明天");
    let s1 = "明天明天明天".to_string();
    println!("s:{}", std::mem::size_of_val(&s));
    println!("s1:{}", std::mem::size_of_val(&s1));
    println!("{}", s);
    println!("{}", &s);
    println!("{}", &s[0..3]);

    println!("{}", std::mem::size_of::<i32>());
    println!("{}", std::mem::size_of::<String>());
    println!("{}", std::mem::size_of_val("明天"));
}

fn main2() {
    println!("-----------");
    println!("Start main2");

    let user = User {
        id: 1,
        name: String::from("james"), // "james".to_string(),
        active: true,
    };
    let user2 = User {
        id: 2,
        name: "peter".to_string(),
        ..user
    };
    println!("{} {} {}", user.id, user.name, user.active);
    println!("{} {} {}", user2.id, user2.name, user2.active);

    // let user3 = build_user(user, 3, "tom".to_string());
    // println!("{} {} {}", user3.id, user3.name, user3.active);

    // println!("{:?}", u`ser3);

    let a = [1,2,3];
    println!("{:?}", a);

    let coord = Coord {
        x: 1,
        y: 2
    };
    println!("coord.add {}", coord.add());
}

fn main() {
    let x: i64 = 5;
    let y = 6;
    let z = x + y;
    let mut acc: i64 = 3;
    acc = acc + 1;
    acc = acc + 4;
    const A: i32 = 1000_0000;
    let b = "     ";
    let god = true;
    let (l, m, n) = (1, 2, 3);
    println!("{} {} {}", l,m,n);
    let p = (1, "2.0", 3.14);
    let arr = [1,2,3,4,5];
    let arr2 = [100; 19_0000];

    println!("{}", arr2.len());
    println!("arr.len() {} {} {}", arr.len(), arr[0], arr[1]);
    println!("{} {} {}", p.0, p.1, p.2);
    println!("{}", !!god);
    println!("{}", b.len());
    println!("{}", A);
    println!("Hello, world!");
    println!("hello world 2");
    println!("{} {} {}", x+8, z, acc);

    bar(10, 20);
    println!("{}", try_one(100));
    try_two();

    main2();
    main3();
    main4();
    main5();
    main6();
    main7();
    main8();
    main9();
}

fn bar(x: i32, y: i32) {
    println!("okok");
    println!("{} {}", x, y);

    let x = 5;

    let y = {
        let x = 3;
        x * x
    };
    println!("{} {}", x, y);
}

fn try_one(x: i32) -> i32 {
    let a = loop {
        println!("{}", x);

        if x == 100 {
            break x * 2;
        }
    };

    println!("a={}", a);
    a
}

fn try_two() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for element in &[1,2,3,4] {
        println!("{}", element);
    }
}