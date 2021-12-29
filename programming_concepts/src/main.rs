fn main() {
    // Variables are immutable by default, but immutable variables differ from
    // constants.
    // Variables are declared using the let keyword, and can be shadowed.
    let x = 5;
    let x = x + 1;
    {
        // Same name, but temporary scope. Doesn't modify x in the outer scope
        let x = x * 2;
        println!("Inner scope: x = {}", x);
    }
    println!("Outer scope: x = {}", x);
    // Variables which you wish we modify need to be declared as mutable
    let mut y = 2;
    y = y + 2;
    println!("y = {}", y);
    // Constants MUST come with a type declaration
    const NUM_SECONDS_PER_HOUR: u32 = 60 * 60;
    const NUM_HOURS_PER_DAY: u32 = 24;
    println!(
        "There are {} seconds in a day",
        NUM_HOURS_PER_DAY * NUM_SECONDS_PER_HOUR
    );

    // Rust has familiar types, including integers (signed and unsigned),
    // floats, bools and chars. It also has two fundamental compound types:
    // tuples and arrays (both have a fixed length).
    let tup = ('t', 42, 3.14); // Tuple elements can vary in type
    let (_, _e1, e2) = tup; // Tuple destructuring. _ prefix --> don't warn if unused
    println!("Tuple elements are: {}, {}, {}", tup.0, tup.1, e2); // Index access
    let _tup2: (i32, char) = (25, 'c'); // Type can also be explicated
                                        // Arrays now. All elements must be of the same type
    let arr: [i32; 5] = [1, -2, 3, -4, 5];
    println!("arr: {:?}", arr);
    println!("First element of arr: {}", arr[0]);
    let arr2 = [0; 10]; // 10 elements, all initialized to 0
    println!("arr2: {:?}", arr2);

    // In Rust, function parameters MUST be type-annotated. Function names are
    // conventionally written in snake case. Rust also distinguishes between
    // expressions and statements. An expression returns a value while a
    // statement doesn't. For example, "let y = 6;" is a statement. So you
    // can't write something like "let x = (let y = 6);", which you *could* do
    // in C. An expression which is terminated in a semicolon becomes a
    // statement. The block used to create new scopes {} is an expression.
    let val = sub_two(0);
    println!("Function returned {}", val);

    // Control flow. In Rust, if statements must always be provided with a bool
    // (Rust will not attempt to perform type conversion).
    // Loops. Rust has 3 types of loops: loop, while and for.
    let mut i = 0;
    let result = 'loop_label: loop {
        // Runs forever
        i = i + 1;
        if i == 10 {
            // Can specify a loop to break out of. By default it'll break the
            // innermost one. Can also return a value from the loop when
            // breaking
            break 'loop_label i;
        }
    };
    println!("Broke out of loop with result {}", result);
    i = 0;
    while i < arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
        i += 1;
    }
    println!("Finished while loop");
    for element in arr {
        println!("{}", element);
    }
    println!("Finished for loop");

    // Conditional assignment. The variable type must be identical for all arms
    // of the control flow logic, otherwise the compiler would not be able to
    // determine the variable's type at compile time (which would limit its
    // ability to make guarantees about usage validity)
    let condition = true;
    let _number = if condition { 7 } else { 8 };
}

fn sub_two(value: i32) -> i32 {
    // In Rust, the return value of a function is synonymous with the value of
    // the final expression in the function
    value - 2 // Needs to be an expression, thus can't terminate with semicolon
}
