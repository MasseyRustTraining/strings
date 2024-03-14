fn strconst() {
    let crab: char = '🦀';
    // let crab = 0x1f980; // (actual crab)
    // let crab = 0x7e_u8 as char;
    // let crab = 0xffff_ffff_u32 as char;
    // let crab = char::from_u32(0xffff_ffff).unwrap();
    println!("{} {:0x}", crab, crab as u32);

    let crab: &str = "🦀";
    println!("{} {}", crab, crab.len());

    // This bit could be
    //      let crab_chars: Vec<char> = crab.chars().collect();
    let mut crab_chars: Vec<char> = Vec::new();
    for c in crab.chars() {
        crab_chars.push(c);
    }

    println!("{}", crab_chars[0]);
}

fn strstuff() {
    let crab_char =  '🦀';
    let mut crab = String::new();
    for _ in 0..5 {
        crab.push(crab_char);
    }
    crab.push('!');
    println!("{} {}", crab, crab.len());
    
    let crab: &str = &crab;
    println!("{} {}", crab, crab.len());
}

fn _drop_string(_s: String) {
    // do nothing
}

fn bang(s: &mut String) -> &str {
    s.push('!');
    s
}

fn main() {
    let x = 7u8;
    let y = x;
    println!("{} {}", x, y);

    strconst();
    strstuff();

    let x: String = "hello".to_string();
    let mut y = x.clone();
    // let yr = &mut y;
    println!("{} {}", x, y);
    let z = bang(&mut y);
    //drop_string(y);
    println!("{}", z);
}
