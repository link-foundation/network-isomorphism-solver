use links_notation::{parse_lino, LiNo};

fn main() {
    // Test parsing simple links notation
    println!("Testing links-notation crate...\n");

    // Simple doublet format
    let input = "(1 2)";
    println!("Input: {}", input);
    match parse_lino(input) {
        Ok(result) => {
            println!("Parsed: {:?}", result);
            explore_lino(&result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // Triplet format
    let input2 = "(A connects B)";
    println!("\nInput: {}", input2);
    match parse_lino(input2) {
        Ok(result) => {
            println!("Parsed: {:?}", result);
            explore_lino(&result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // Named link
    let input3 = "(link1: 1 2)";
    println!("\nInput: {}", input3);
    match parse_lino(input3) {
        Ok(result) => {
            println!("Parsed: {:?}", result);
            explore_lino(&result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // Multiple links
    let input4 = "(1 2) (2 3) (3 1)";
    println!("\nInput: {}", input4);
    match parse_lino(input4) {
        Ok(result) => {
            println!("Parsed: {:?}", result);
            explore_lino(&result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn explore_lino(lino: &LiNo<String>) {
    println!("  is_link: {}", lino.is_link());
    println!("  is_ref: {}", lino.is_ref());

    match lino {
        LiNo::Ref(r) => {
            println!("  -> Ref: {:?}", r);
        }
        LiNo::Link { id, values } => {
            println!("  -> Link id: {:?}, values count: {}", id, values.len());
            for (i, val) in values.iter().enumerate() {
                println!("     value[{}]: {:?}", i, val);
            }
        }
    }
}
