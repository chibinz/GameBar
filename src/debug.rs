pub fn debug()
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let command: Vec<&str> = input.split_whitespace().collect();

    match command[0]
    {
        "print"       => println!("Hello world"),
        "disassemble" => println!("Hello world!"),
        _ => (),
    };
}