use std::fs::File;
use std::io;
use std::io::Read;
pub fn mynulltype(){


    let a = Some(String::from("az"));
    assert_eq!(a.unwrap(), "a");


}


pub fn test_open_file(){
    let file_result = File::open("/tmp/foo");



    let result = match file_result {
        Ok(file) => {
            println!("{:?}",file);
            file
        }
        Err(error) => {
            panic!("{:?}", error)
        }
    };

}


pub fn read_username_from_file()->Result<String,io::Error>{
    let mut file = File::open("/tmp/foo")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let a = "100".parse::<i32>()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    Ok(contents)
}