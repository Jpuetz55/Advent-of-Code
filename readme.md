from Day 2 - Note code snippet:

let color_char: char = game.chars().nth((i + 2).try_into().unwrap()).unwrap();

game.chars(): This part converts the game string into an iterator of characters (Chars). The chars() method is used to obtain an iterator over Unicode characters.

nth((i + 2).try_into().unwrap()): This method is called on the iterator of characters (Chars). It retrieves the element at the specified index, where the index is calculated as (i + 2). The try_into() method is used to convert the resulting index to the appropriate type, and unwrap() is used to handle the Result (or Option in this case) and retrieve the index value.

unwrap(): This is used twice in this expression. The first unwrap() is used to unwrap the Option returned by nth. If the index is out of bounds, it will return None, and unwrap() is used to get the actual character. The second unwrap() is used to unwrap the Result returned by try_into(). If the conversion fails, it will panic.

Putting it all together, this code retrieves the character at the index (i + 2) from the game string and assigns it to the variable color_char. It assumes that the index calculation and conversions are successful, and the unwraps are used to handle these conversions. Keep in mind that using unwrap() in production code might lead to panics, so error handling might be more appropriate in a robust application.
