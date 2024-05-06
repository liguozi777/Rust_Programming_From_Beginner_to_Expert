fn main() {
    // 整数类型
    let integer1: u32 = 17; // 类型声明
    let integer2 = 17u32; // 类型后缀声明
    let integer3 = 17; // 默认i32类型
    let integer4: u32 = 0b10001; // 二进制
    let integer5: u32 = 0o21; // 八进制
    let integer6: u32 = 0x11; // 十六进制
    let integer7 = 50_000; // 数字可读性分割符_

    println!("{}", integer1);
    println!("{}", integer2);
    println!("{}", integer3);
    println!("{}", integer4);
    println!("{}", integer5);
    println!("{}", integer6);
    println!("{}", integer7);

    // 浮点数类型
    let float1: f32 = 1.1; // 类型声明
    let float2 = 2.2f32; // 类型后缀声明
    let float3 = 3.3; // 默认f64类型
    let float4 = 11_000.555_001; // 数字可读性分隔符

    println!("{}", float1);
    println!("{}", float2);
    println!("{}", float3);
    println!("{}", float4);

    // 布尔类型
    let t: bool = true; // 显式类型声明
    let f = false; // 隐式类型声明

    println!("{}", t);
    println!("{}", f);

    // 字符类型
    let z = 'z';
    let zz = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", z);
    println!("{}", zz);
    println!("{}", heart_eyed_cat);

    // 范围类型
    // 左闭右开区间
    print!("(1..5):");
    for i in 1..5 {
        print!("{}", i);
    }
    println!();

    // 全闭区间
    // rev()方法用于范围内的数字顺序反转
    print!("(1..=5).rev:");
    for i in (1..=5).rev() {
        print!("{}", i);
    }
    println!();

    // sum()方法用于范围内的数字求和
    let sum: i32 = (1..=5).sum();
    println!("1 + 2 + 3 + 4 + 5 = {}", sum);
}
