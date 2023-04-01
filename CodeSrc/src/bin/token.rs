


fn main() {
    println!("这是示例宏中对于token解析的特殊语法使用的参考。");

    assert_eq!(测试函数1(), 测试函数2());

    测试函数3();
    测试函数4();
    测试函数5();
    测试函数6();
}

    /*
    带有任意后缀的字面值token可以传递给宏而不产生错误。
    宏本身将决定如何解释这样的token以及是否产生错误。
    特别是，示例宏的 `literal` 片段 specifier "指示器"可以匹配具有任意后缀的字面值token。
    */
macro_rules! blackhole { ($tt:tt) => () }
macro_rules! blackhole_lit { ($l:literal) => () }

fn 测试函数1(){
    blackhole!("string"suffix);     // 这里的字面值 "string" 添加了后缀，在示例宏中是允许的。
}

fn 测试函数2(){
    blackhole_lit!(1suffix);        // 这里的字面值 1 添加了后缀，在示例宏中是允许的。
}

fn 测试函数3(){
    // 当一个未转义的 `U+005C`  (`\`) 字符出现在换行之前，那么忽略断行符和所有紧随其后的 ` ` (`U+0020`) 、 `\t` (`U+0009`) 、 `\n` (`U+000A`) 、 `\r` (`U+0000D`) 字符。因此 `a` 、 `b` 、 `c` 相同:
    let a = "foobar";
    let b = "foo\
            bar";
    let c = "foo\

        bar";

    assert_eq!(a, b);  // 即 a 与 b 等价
    assert_eq!(b, c);  // 即 b 与 c 等价
}

fn 测试函数4(){
    // 原始字面量以字符 `U+0072` (`r`) 开始，后面是少于256个的 `U+0023` (`#`) 和 `U+0022` (双引号) 字符。
    println!("{} {}", "foo" , r"foo" ); // foo
    println!("{} {}",  "\"foo\"", r#""foo""# ); // "foo"
    println!("{} {}","foo #\"# bar" , r##"foo #"# bar"## ); // foo #"# bar
    println!("{} {} {}",  "\x52" , "R" , r"R" );  // R
    println!("{} {}",  "\\x52" , r"\x52" ); // \x52

    // 原始字节字符串字面值不处理任何转义。以字符 `U+0062` (`b`) 开始，然后是 `U+0072` (`r`) ，后面是少于256的字符 `U+0023` (`#`)，以及一个 `U+0022` (双引号) 字符。
    println!("{:?} {:?}", b"foo" , br"foo" );  // foo
    println!("{:?} {:?}", b"\"foo\"" , br#""foo""# ); // "foo"
    println!("{:?} ", b"foo #\"# bar" );    // foo #"# bar
    println!("{:?} ", br##"foo #"# bar"## ); // foo #"# bar
    println!("{:?} {:?} {:?}", b"\x52" , b"R", br"R"); // R
    println!("{:?} {:?}", b"\\x52" , br"\x52" ); // \x52

}

fn 测试函数5(){
    // 各类数值字在值

    assert_eq!(123, 123i32); 
    assert_eq!(123u32, 123_u32); 

    assert_eq!(0xff, 0xff_u8); 
    assert_eq!(0x01_f32, 7986); 
    assert_eq!(0x01_e3, 483); 

    assert_eq!(0o70, 0o70_i16); 

    assert_eq!(0b1111_1111_1001_0000, 0b1111_1111_1001_0000i64); 

    println!("0b________1 {}",  0b________1 ); 

    println!("0usize {}", 0usize ); 
    
    println!("5f32 {}", 5f32 ); 
}

fn 测试函数6(){
    // 元组索引

    let example = ("dog", "cat", "horse");
    let dog = example.0;
    let cat = example.1;

    println!(" {} {} ", dog, cat ); 

    // 下面的例子是无效的。
    // let cat = example.01;  // ERROR 字段名不能为 `01`
    // let horse = example.0b10;  // ERROR 字段名不能为 `0b10`
}