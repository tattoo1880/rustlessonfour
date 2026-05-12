mod mylifetime;
use mylifetime::imylifetime;
use mylifetime::usenulltype;


macro_rules! add {
    ($a: expr,$b: expr) => {
        $a + $b
    };
}

fn main() {
    // imylifetime::usefoo();
    // usenulltype::mynulltype();
    // usenulltype::test_open_file();
    let result = usenulltype::read_username_from_file();
    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
    let rs = add!(1, 2);
    println!("{}", rs);
}
