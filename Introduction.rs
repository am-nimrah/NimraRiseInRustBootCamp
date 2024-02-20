// concatenate_strings Function Defination with string return type and 2 parameter strings
fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new(); //created result variable to hold concatenated results
    
    result.push_str(str1);//appended string 1 to result string
    
    result.push_str(str2);//appended string 2 to result string

    result //returning result string after concatenation
}

fn main() {
    let string1 = String::from("Hi! I am Nimra Waqar."); //definition and initialization if string 1
    let string2 = String::from("I am a ML Engineer and passionate about NLP."); //definition and initialization if string 2
    
    let concatenated_string = concatenate_strings(&string1, &string2);// calling the above funtion with the references of above parameters and storing the result of function in a string 
    
    println!("{}", concatenated_string);//printing the results stored in the string
}
