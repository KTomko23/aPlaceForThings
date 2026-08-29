use std::str;

fn main() 
{
    println!("Hello, world!");



    let x = 10;
    let y = 5;

    let mut v = 5;

    println!("x: {}, y: {}, v: {}", x, y, v);

    let sum = add(x, y);
    println!("Sum: {}", sum);



    let mut my_thing = things {
        name: String::from("My Thing"),
        value: sum
    };

    println!("Name: {}, Value: {}", my_thing.name, my_thing.value);


    let a: u8 = 0b0001_1010;

    println!("a << 1 = {:08b}", a << 1); // 00110100
    println!("a >> 2 = {:08b}", a >> 2); 


    let b: u8 = 0b1011_0110;

    let low_nibble = b & 0b0000_1111;
    let high_nibble = (b & 0b1111_0000) >> 4;

    println!("low  nibble: {:04b}", low_nibble);
    println!("high nibble: {:04b}", high_nibble);

    let value: u32 = 0x12345678;

    let b0 = (value & 0x000000FF) >> 0;
    let b1 = (value & 0x0000FF00) >> 8;
    let b2 = (value & 0x00FF0000) >> 16;
    let b3 = (value & 0xFF000000) >> 24;

    println!("bytes: {:02X} {:02X} {:02X} {:02X}", b3, b2, b1, b0);

    let mut data = [0u8; 4];

    data[0] = 0x12;
    data[1] = 0x34;
    data[2] = 0x56;
    data[3] = 0x78;

    println!("{:02X?}", data);



    //malloc stuff

    //int *b = malloc(sizeof(int));
    let b = Box::new(42);

    //int *arr = malloc(sizeof(int) * n);
    let mut v = Vec::with_capacity(10);
    v.push(1);
    v.push(2);


    //char *s = malloc(...);
    let s = String::from("m.a.l.l.o.c");

    println!("s: {}",s);

}


fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct things {
    name: String,
    value: i32,
}