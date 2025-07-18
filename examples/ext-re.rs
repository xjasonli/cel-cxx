#![allow(clippy::uninlined_format_args)]

use cel_cxx::*;

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<(), Error> {
    // Enable C++ regular expression extension
    let env = Env::builder()
        .with_ext_re(true)
        .build()?;

    println!("=== CEL C++ Regular Expression Extension (re) Examples ===\n");

    // 1. re.extract - Extract and rewrite matched groups
    println!("1. re.extract Examples:");
    
    // Swap word order
    let program = env.compile(r#"re.extract("Hello World", r"(\w+) (\w+)", r"\2, \1")"#)?;
    let result = program.evaluate(())?;
    println!("   re.extract(\"Hello World\", r\"(\\w+) (\\w+)\", r\"\\2, \\1\") = {result}");
    
    // Date format conversion
    let program = env.compile(r#"re.extract("2023-12-25", r"(\d{4})-(\d{2})-(\d{2})", r"\2/\3/\1")"#)?;
    let result = program.evaluate(())?;
    println!("   re.extract(\"2023-12-25\", r\"(\\d{{4}})-(\\d{{2}})-(\\d{{2}})\", r\"\\2/\\3/\\1\") = {result}");
    
    // Extract and format URL
    let program = env.compile(r#"re.extract("https://example.com/path", r"https://([^/]+)(.*)", r"Domain: \1, Path: \2")"#)?;
    let result = program.evaluate(())?;
    println!("   URL extraction: {result}");
    
    println!();

    // 2. re.capture - Capture the first group
    println!("2. re.capture Examples:");
    
    // Extract username from email
    let program = env.compile(r#"re.capture("user@example.com", r"(\w+)@[\w.]+")"#)?;
    let result = program.evaluate(())?;
    println!("   re.capture(\"user@example.com\", r\"(\\w+)@[\\w.]+\") = {result}");
    
    // Extract number from price
    let program = env.compile(r#"re.capture("Price: $29.99", r"Price: \$(\d+\.\d+)")"#)?;
    let result = program.evaluate(())?;
    println!("   re.capture(\"Price: $29.99\", r\"Price: \\$(\\d+\\.\\d+)\") = {result}");
    
    // Extract log level
    let program = env.compile(r#"re.capture("[ERROR] Something went wrong", r"\[(\w+)\].*")"#)?;
    let result = program.evaluate(())?;
    println!("   re.capture(\"[ERROR] Something went wrong\", r\"\\[(\\w+)\\].*\") = {result}");
    
    println!();

    // 3. re.captureN - Capture all groups
    println!("3. re.captureN Examples:");
    
    // Capture email username and domain
    let program = env.compile(r#"re.captureN("user@example.com", r"(\w+)@([\w.]+)")"#)?;
    let result = program.evaluate(())?;
    println!("   re.captureN(\"user@example.com\", r\"(\\w+)@([\\w.]+)\") = {result}");
    
    // Use named groups to capture name
    let program = env.compile(r#"re.captureN("John Doe", r"(?P<first>\w+) (?P<last>\w+)")"#)?;
    let result = program.evaluate(())?;
    println!("   re.captureN(\"John Doe\", r\"(?P<first>\\w+) (?P<last>\\w+)\") = {result}");
    
    // Capture date components
    let program = env.compile(r#"re.captureN("2023-12-25", r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})")"#)?;
    let result = program.evaluate(())?;
    println!("   re.captureN(\"2023-12-25\", r\"(?P<year>\\d{{4}})-(?P<month>\\d{{2}})-(?P<day>\\d{{2}})\") = {}", result);
    
    // Mixed named and unnamed groups
    let program = env.compile(r#"re.captureN("Version 1.2.3", r"Version (?P<major>\d+)\.(\d+)\.(\d+)")"#)?;
    let result = program.evaluate(())?;
    println!("   re.captureN(\"Version 1.2.3\", r\"Version (?P<major>\\d+)\\.(\\d+)\\.(\\d+)\") = {}", result);
    
    println!();

    // 4. Real-world use cases
    println!("4. Real-world Use Cases:");
    
    // Log parsing
    let program = env.compile(r#"re.captureN("[2023-12-25 10:30:45] INFO: User login successful", r"\[([^\]]+)\] (\w+): (.+)")"#)?;
    let result = program.evaluate(())?;
    println!("   Log parsing: {}", result);
    
    // Phone number formatting
    let program = env.compile(r#"re.extract("(555) 123-4567", r"\((\d{3})\) (\d{3})-(\d{4})", r"\1-\2-\3")"#)?;
    let result = program.evaluate(())?;
    println!("   Phone formatting: {}", result);
    
    // URL component extraction
    let program = env.compile(r#"re.captureN("https://api.example.com:8080/v1/users?id=123", r"https://([^:]+):(\d+)(/[^?]+)\?(.+)")"#)?;
    let result = program.evaluate(())?;
    println!("   URL components: {}", result);
    
    println!();

    // 5. Error handling examples
    println!("5. Error Handling:");
    
    // Invalid regular expression
    match env.compile(r#"re.extract("test", "[", "replacement")"#) {
        Ok(program) => {
            match program.evaluate(()) {
                Ok(result) => println!("   Unexpected success: {}", result),
                Err(e) => println!("   Invalid regex error: {}", e),
            }
        }
        Err(e) => println!("   Compile error: {}", e),
    }
    
    // Pattern that doesn't match
    match env.compile(r#"re.capture("no numbers here", r"(\d+)")"#) {
        Ok(program) => {
            match program.evaluate(()) {
                Ok(result) => println!("   Unexpected success: {}", result),
                Err(e) => println!("   No match error: {}", e),
            }
        }
        Err(e) => println!("   Compile error: {}", e),
    }
    
    // No capturing groups
    match env.compile(r#"re.captureN("test", r"test")"#) {
        Ok(program) => {
            match program.evaluate(()) {
                Ok(result) => println!("   Unexpected success: {}", result),
                Err(e) => println!("   No capturing groups error: {}", e),
            }
        }
        Err(e) => println!("   Compile error: {}", e),
    }

    println!("\n=== All re extension examples completed! ===");
    Ok(())
} 