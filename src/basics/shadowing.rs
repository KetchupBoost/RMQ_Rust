fn shadow() {
    let x = 5;

    let x = x + 1;
    
    // Inner Scope
    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    
    // For example, say our program asks a user to show
    // how many spaces they want between some text by inputting space characters, 
    // and then we want to store that input as a number:
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");
    // if we try to use mut for this, as shown here, 
    // we’ll get a compile-time error, 
    // the error says we’re not allowed to mutate a variable’s type.
}