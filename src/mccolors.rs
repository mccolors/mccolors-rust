// Function to map Minecraft color codes to ANSI escape codes
fn map_minecraft_color_to_ansi(color_code: char) -> &'static str {
    match color_code {
        '0' => "\x1b[30m", // Black
        '1' => "\x1b[34m", // Blue
        '2' => "\x1b[32m", // Green
        '3' => "\x1b[36m", // Cyan
        '4' => "\x1b[31m", // Red
        '5' => "\x1b[35m", // Magenta
        '6' => "\x1b[33m", // Yellow
        '7' => "\x1b[38;5;250m", // White
        '8' => "\x1b[90m", // Dark Gray
        '9' => "\x1b[94m", // Light Blue
        'a' => "\x1b[92m", // Light Green
        'b' => "\x1b[96m", // Light Cyan
        'c' => "\x1b[91m", // Light Red
        'd' => "\x1b[35;1m", // Bold Magenta
        'e' => "\x1b[93m", // Light Yellow
        'f' => "\x1b[37m", // White
        'l' => "\x1b[1m", // Bold
        'm' => "\x1b[9m", // Strike-through
        'n' => "\x1b[4m", // Underline
        'o' => "\x1b[3m", // Italic
        'r' => "\x1b[0m", // Reset to default
        _ => "", // If it doesn't match any, do nothing
    }
}

// Function to convert Minecraft-formatted text to ANSI-formatted text and print
pub fn mcwrite(input: &str) {
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '&' {
            if let Some(color_code) = iter.peek() {
                let ansi_code = map_minecraft_color_to_ansi(*color_code);
                print!("{}", ansi_code);
                iter.next(); // Consume the next character (the color code in Minecraft)
            }
        } else {
            print!("{}", c);
        }
    }

    // Restore to default values at the end
    print!("\x1b[0m");
    println!();
}

// Function to convert Minecraft-formatted text to ANSI-formatted text
pub fn mcreplace(input: &str) -> String {
    let mut output = String::new();
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '&' {
            if let Some(color_code) = iter.peek() {
                let ansi_code = map_minecraft_color_to_ansi(*color_code);
                output.push_str(ansi_code);
                iter.next(); // Consume the next character (the color code in Minecraft)
            }
        } else {
            output.push(c);
        }
    }

    // Restore to default values at the end
    output.push_str("\x1b[0m");
    output
}

// Function to delete Minecraft color codes from text
pub fn mcdelete(input: &str) -> String {
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
