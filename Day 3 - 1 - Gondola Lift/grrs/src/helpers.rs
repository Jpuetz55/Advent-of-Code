//parse int from string
//given string slice starting with a number, will parse digit out of string reutrning value of digit upon encountering a non digit character


pub fn parse_int_from_string(s: String) -> <u32,i32>  { 
       
    let mut i = 0;

    digits.push(letter.to_digit(10).unwrap())
    if !s.chars().nth((i + 1).try_into().unwrap()).unwrap().is_digit(10) //next character sin't a digit. we have all digits now.
    { 
            let mut temp_total = 0;
            let mut mult_casc = 1;
            let mut temp = 0;                                      //iterate backwards on vec, aka, starting from the ones and multiply multiplication factor by 10 for each digit. add results together
            for &element in digits.iter().rev() {
                temp = element * mult_casc;
                //multiply by mult_cascade
                temp_total += temp;
                //multiply mult_casc by 10
                mult_casc *= 10;                         
            }
            temp_total
    }
    i +=1;
}
                    