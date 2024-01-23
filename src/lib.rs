fn map_minecraft_color_to_ansi(color_code: char) -> &'static str {
    match color_code {
        '0' => "\x1b[30m",       // Black
        '1' => "\x1b[34m",       // Blue
        '2' => "\x1b[32m",       // Green
        '3' => "\x1b[36m",       // Cyan
        '4' => "\x1b[31m",       // Red
        '5' => "\x1b[35m",       // Magenta
        '6' => "\x1b[33m",       // Yellow
        '7' => "\x1b[38;5;250m", // White
        '8' => "\x1b[90m",       // Dark Gray
        '9' => "\x1b[94m",       // Light Blue
        'a' => "\x1b[92m",       // Light Green
        'b' => "\x1b[96m",       // Light Cyan
        'c' => "\x1b[91m",       // Light Red
        'd' => "\x1b[35;1m",     // Bold Magenta
        'e' => "\x1b[93m",       // Light Yellow
        'f' => "\x1b[37m",       // White
        'l' => "\x1b[1m",        // Bold
        'm' => "\x1b[9m",        // Strike-through
        'n' => "\x1b[4m",        // Underline
        'o' => "\x1b[3m",        // Italic
        'r' => "\x1b[0m",        // Reset to default
        _ => "",                 // If it doesn't match any, do nothing
    }
}

pub fn mcreplace(input: &str) -> String {
    // Create a new mutable string to store the result
    let mut output = String::new();

    // Create an iterator over the characters of the input with peekable to be able to look at the next character without advancing the iterator
    let mut chars = input.chars().peekable();

    // Iterate over the characters of the input
    while let Some(c) = chars.next() {
        // Check if the current character is '&'
        if c == '&' {
            // Try to get the next character from the iterator
            if let Some(color_code) = chars.next() {
                // Call the map_minecraft_color_to_ansi function to get the corresponding ANSI sequence for the Minecraft color code
                let ansi_sequence = map_minecraft_color_to_ansi(color_code);

                // Add the ANSI sequence to the result
                output.push_str(ansi_sequence);
            }
        } else {
            // If the character is not '&', simply add it to the result
            output.push(c);
        }
    }

    // Reset ANSI formatting before returning the resulting string
    output.push_str("\x1b[0m");

    // Return the resulting string with Minecraft color codes replaced by ANSI codes
    output
}

pub fn mcwrite(input: &str) {
    // Create an iterator over the characters of the input with peekable to be able to look at the next character without advancing the iterator
    let mut chars = input.chars().peekable();

    // Iterate over the characters of the input
    while let Some(c) = chars.next() {
        // Check if the current character is '&'
        if c == '&' {
            // Try to get the next character from the iterator
            if let Some(color_code) = chars.next() {
                // Call the map_minecraft_color_to_ansi function to get the corresponding ANSI sequence for the Minecraft color code
                let ansi_sequence = map_minecraft_color_to_ansi(color_code);

                // Print the ANSI sequence directly to the console
                print!("{}", ansi_sequence);
            }
        } else {
            // If the character is not '&', simply print it to the console
            print!("{}", c);
        }
    }

    // Reset ANSI formatting before finishing
    print!("\x1b[0m");
}

pub fn mcremove(input: &str) -> String {
    let mut output = String::new();
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '&' {
            // Skip the color code character
            iter.next();
        } else {
            output.push(c);
        }
    }

    output
}
