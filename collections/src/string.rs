// UTF-8(8-bit Unicode Transformation Format)是一种针对 Unicode 的可变长度字元编码，是一种前缀码，
// 它可以用一至四个字节对 Unicode 字符集中的所有有效编码点进行编码，属于 Unicode 标准的一部分。
// Unicode 编码（每个字符占用 2 个字节，即 16-bit）
// strings are implemented as a collection of bytes
// The String type, which is provided by Rust's std library rather then coded into the core language,
// is a growable, mutable, owned, UTF-8 encoded string type.
// Rust std library also includes a number of other string types, such as OsString, OsStr, CString, CStr

use unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // String are UTF-8 encoded, we can include any properly encoded data in them
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s += "bar";
    println!("{}", s);
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    let mut s = String::from("foo");
    s.push('b');
    s.push('a');
    s.push('r');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note that s1 has been moved here
    println!("{}", s3);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", &s1, &s2);
    println!("{}", s3);
    println!("{}{}", s1, s2);

    let s1 = String::from("hello");
    // let h = s1[0]; // strings don't support indexing, because it's UTF-8 encode,
    // which one character may take 1-4 bytes and strings are collections of bytes,
    // so index one byte may make not sense if it don't point to the byte that exactly can represent one character
    // a String is a wrapper of Vec<u8> vector of unsigned 8-bit is vector of bytes
    let len = String::from("Hola").len();
    println!("{}", len); // 4
    let len = String::from("Здравствуйте").len();
    println!("{}", len); // 24

    // grapheme: 字素，书写系统的最小有意义单位
    // another pointer about UTF-8 is that there are actually three relevant ways to look at strings
    // from Rust perspective: as bytes, as scalar values, and grapheme cluster(or we called letters)
    let s = String::from("नमस्ते");
    // as bytes
    println!("{:?}", s.as_bytes());
    // as scalar chars, Rust char take 4 bytes
    println!("{:?}", s.chars());
    // as grapheme clusters
    println!(
        "{:?}",
        UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<&str>>()
    )

    // a final reason Rust doesn't allow to index into a string to get a character is that
    // indexing operations are expected to always constant time(O(1)).
    // But it isn't possible to guarantee that performance with a string, because Rust would have to
    // walk through the contents from the beginning to the index to determine how many valid characters there

    // Rust will panic at runtime if the string slice [a..b] has an invalid index
    // which was not end of grapheme cluster in vector
}
