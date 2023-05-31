fn array() {
    // arrays are more useful when you know the number of elements
    // will not need to change

    // [type; n]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];

    // You can also initialize an array
    // to contain the same value for each element by 
    // specifying the initial value, followed by a semicolon,
    // and then the length of the array in square brackets, 
    // as shown here:
    let a = [3; 5];
}