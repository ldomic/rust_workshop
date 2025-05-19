fn single_str(s: &str) -> &str {
    // only one ref - lifetime elison kicks in
    s
}

fn combine<'a>(x: &'a str, y: &'a str) -> &'a str {
    // more than one input reference - need to ensure that returned
    // value lives at least as long as the shortest input ref
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// uncomment to see the error

// fn combine(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

pub fn get_combined_str() {
    let string1 = String::from("Rust");
    let string2 = String::from("=== awesome!");
    let string3 = String::from("Your Rust code ");

    let single = single_str(&string3);
    println!("The single string is: {}", single);

    // what's the outcome?
    let combined = combine(&string1, &string2);
    println!("The combined string is: {}", combined);

    println!("Therefore: {}{}", single, combined);
}

// Structs and lifetimes
// In this example, we have a struct `Document` that contains a reference to a string slice
// and an owned string. The lifetime of the reference is tied to the lifetime of the struct.

struct Document<'a> {
    content: &'a str,
    // author is value owned by struct, hence it doesn't need lifetime
    author: String,
}

impl<'a> Document<'a> {
    fn new(content: &'a str, author: String) -> Self {
        Document { content, author }
    }

    fn display(&self) {
        println!(
            "This document '{}' was created by {}",
            self.content, self.author
        );
    }
}

pub fn create_document() {
    let string = String::from("Rust lifetimes workshop");
    let author = String::from("Laura Sobola");
    let doc = Document::new(&string, author);
    doc.display();
}

// Traits and lifetimes
// Pretty much the same as structs, but we need to specify the lifetime
// in the trait definition as well. This is because the trait can be implemented
// for different types, and we need to ensure that the lifetime is consistent
// across all implementations.

trait Extract<'a> {
    fn extract_content(&self) -> &'a str;
}

impl<'a> Extract<'a> for Document<'a> {
    fn extract_content(&self) -> &'a str {
        self.content
    }
}

pub fn get_document_extract() {
    let doc = Document {
        content: "Important Rust document",
        author: String::from("Laura Sobola"),
    };
    println!("Extracted: {}", doc.extract_content());
}

// combining what we have learned so far

// lifetime 'a is necessary as it deals with the `data` reference
// in function new - data is a part of the struct itself
struct Wrapper<'a> {
    data: &'a str,
}

impl<'a> Wrapper<'a> {
    fn new(data: &'a str) -> Self {
        Wrapper { data }
    }

    // 'other' is a reference external to the struct and therefore not covered by lifetime 'a
    // however whatever the implemented function returns has to survive at least as long as
    // struct itself - the separate lifetime relationship is denoted by 'b

    // run the code from tutorials, folks!

    fn combine<'b>(&self, other: &'b str) -> &'a str
    // 'b outlives 'a (lifetime bound)
    where
        'b: 'a,
    {
        if self.data.len() > other.len() {
            self.data
        } else {
            other
        }
    }
}

pub fn wrap_data() {
    // try changing string1 and string2 to see a difference in results
    // is the lifetime bound always necessary?
    let string1 = String::from("hello");
    let string2 = "world";
    let wrapper = Wrapper::new(&string1);
    let result = wrapper.combine(string2);
    println!("Combined: {}", result);
}
