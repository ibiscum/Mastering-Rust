fn main() {
    let level_0_str = String::from("foo");
    println!("level_0_str is alive: {}", level_0_str);

    {
        let level_1_number = 9;
        println!("Inside level 1 block, number = {}", level_1_number);

        {
            let level_2_vector = vec![1, 2, 3];
            println!("Inside level 2 block, vector = {:?}", level_2_vector);
        } // level_2_vector goes out of scope here
        println!("Back in level 1 block, number still = {}", level_1_number);

        {
            let level_2_number = 9;
            println!("Inside another level 2 block, number = {}", level_2_number);
        } // level_2_number goes out of scope here
    } // level_1_number goes out of scope here

    println!("Back in main, level_0_str still alive: {}", level_0_str);
} // level_0_str goes out of scope here
