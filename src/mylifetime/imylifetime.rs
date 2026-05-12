pub fn imml_lifetime() {
    println!("Imml lifetime");
}



pub fn print_str(){
    let s = "hello world".to_string();
    let a = &s;
    println!("{}", a);
}



static ASTR: &'static str = "hello world";
fn foo()->&'static str{

    ASTR
}


pub fn usefoo(){
    let s = foo();
}