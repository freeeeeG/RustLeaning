

fn main() {
    // 所有权
    // 创建一个 String 对象并获得所有权
    let s1= String::from("Hello, Rust!");

    // 将所有权转移到新的变量 s2
    let s2 = s1;

    // 以下代码将无效，因为所有权已经转移给 s2
    // println!("{}", s1); // 这行代码将产生编译错误
    
    println!("{}", s2); // 可以正常打印 s2 的值
    let s3= &s2; // 借用s2的所有权
    
    // 生命周期
    let s1 = "Hello";
    let s2 = "Rust";

    let holder = create_string_holder(s1);
    println!("{}", holder.value); // 可以正常打印 holder 的值

    let longest = choose_longest(s1, s2);
    println!("Longest: {}", longest); // 可以正常打印 longest 的值
}



// 定义一个结构体，包含一个字符串切片引用
struct StringHolder<'a> {
    value: &'a str,
}

// 函数接受一个字符串切片引用，并返回一个 StringHolder 结构体
fn create_string_holder<'a>(s: &'a str) -> StringHolder<'a> {
    StringHolder { value: s }
}

// 函数接受两个字符串切片引用，并返回一个较长的引用
fn choose_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

