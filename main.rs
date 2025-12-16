fn main() {
    println!("wiist du eine datei lesen (1) oder etwas bearbeiten (2)");
    let mut a= String::new();
    io::stdin().read_line(&mut a).expect("failed to readline");
    let a = a.trim();
    let userinput:i32 = a.parse().unwrap();
    println!("{}", userinput);
    let contents;

    if(userinput == 1 ) {
        lesen(1);
    }
    if ( userinput == 2 ) {
        bearbeiten(userinput :i32, contents:&str );
    }


    let liste = OpenOptions::new().read(true).write(true).create(true).open("todoliste.txt");
    println!("a");
}
fn lesen( userinput :i32 ) {

    println!("z");
    let ausgabe = OpenOptions::new().read(true).open("todoliste.txt");
    match read_to_string("todoliste.txt") {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}


fn bearbeiten( userinput :i32, contents: &str ) -> Result<()> {
    println!("q");
    let mut file = File::create("todoliste.txt")?;
    file.write_all(contents.as_bytes());
    match read_to_string("todoliste.txt") {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}


/*fn bearbeiten ( userinput :i32 ) {
    println!("s")
}
*/
