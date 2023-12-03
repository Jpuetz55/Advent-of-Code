// ---------------------------------------------------------------- Day 3-1, Gondola Lift ----------------------------------------------------------------


fn main() {

    //have to think of each number being in the center of a 5x5 matrix
    //all 3 digit number
    // EX.

    // X X X X X
    // x 1 1 1 x
    // x x x x x

    //if there is anything other than a period in the x cells, then the number in the middle is
    //a part number and should be added to the sum

    //all lines are equal length, so I should be able to calculate and check the location of each cell
    //in the 5 x 5 matrix by moving forward or backward in the file a set amount for each position

    //EX. with the very center cell considered as the current anchor.

    // (i-(backward)) (i-(backward + 1)) (i-(backward + 2)) (i-(backward + 3)) (i-(backward + 4))
    //      (i-2)              1                  1                   1                (i+2)
    // (i+(forward)) (i+(forward + 1)) (i+(forward + 2)) (i+(forward + 3)) (i+(forward + 4))

    //once the values for forward and backward are obtained. Need to have algo to: 
        //detect and number - first digit
            //reads it's value
                //set letter point to middle number
                    //check closest values, then top and bottom from left to right
                        //store all the values that the letter pointer needs to moved by in a array of signed ints in desired order
                        //EX. [(-2),(+2), -(backward), -(backward + 1), -(backward + 2), -(backward + 3), -(backward + 4)
                        //                 (forward), (forward + 1), (forward + 2), (forward + 3), (forward + 4) ]
                        //if a non period is detected
                            //set letter pointer to next number 
                            //break 
                        //if all periods around number
                            //calculate numbers value
                                //set letter point to first digit in number
                                //implement algo from previous excercise to calculate value and add it to total -- make into helper function -- make module?
                            //set letter pointer to next number
                    


    println!("Hello, world!");
}
