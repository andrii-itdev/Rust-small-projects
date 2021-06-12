
fn readline() -> String {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Error::Couldn't read the input");
    input
}

enum LetterType {
    Consonant(char),
    Vowel,
    None
}

fn is_vowel(ch : &char) -> bool {
    let lower = ch.to_lowercase().to_string();
    return lower == "a" || lower == "o" || lower == "i" 
            || lower == "u" || lower == "y" || lower == "e";
}

fn get_letter_type(ch : &char) -> LetterType {
    if is_vowel(ch)
    {
        LetterType::Vowel
    }
    else {
        LetterType::Consonant(*ch)
    }
}

fn process_alphabetic(is_prev_chr_alphabetic : &mut bool, ch : &char,
        pg_string : &mut String, first_letter_type : &mut LetterType) -> bool
    {
    if ch.is_alphabetic() {
        if *is_prev_chr_alphabetic {
            pg_string.push(*ch);
        }
        else {
            *first_letter_type = get_letter_type(&ch); 

            if let LetterType::Vowel = first_letter_type {
                pg_string.push(*ch);
            }
        }
        *is_prev_chr_alphabetic = true;
        true
    }
    else { false }
}

fn process_non_alphabetic(is_prev_chr_alphabetic : &mut bool, ch : &char,
    pg_string : &mut String, first_letter_type : &mut LetterType) {
        if *is_prev_chr_alphabetic {
            
            pg_string.push('-');
            match first_letter_type {
                LetterType::Consonant(ch) => {
                    pg_string.push(*ch);
                    pg_string.push_str("ay");
                },
                _ => {
                    pg_string.push_str("hay");
                }
            }
        }
        pg_string.push(*ch);

        *is_prev_chr_alphabetic = false;

}

fn convert_to_piglatin1(s : &String) -> String {
    let chars = s.chars();
    let mut pg_string : String = String::new();
    let mut is_prev_chr_alphabetic = false;
    let mut first_letter_type : LetterType = LetterType::None;

    for i in chars {
        if !process_alphabetic(&mut is_prev_chr_alphabetic, &i, &mut pg_string, &mut first_letter_type) {
            process_non_alphabetic(&mut is_prev_chr_alphabetic, &i, &mut pg_string, &mut first_letter_type);
        }
    }
    pg_string
}

fn convert_word_to_piglatin(w : &String) -> String {
    let mut word_piglatin : String;
    let f_ch = w.chars().next();
    if let Option::Some(ch) = f_ch {
        if ch.is_alphabetic() {
            if is_vowel(&ch) {
                word_piglatin = w.clone().to_string();
                word_piglatin.push_str("-hay");
            }
            else {
                word_piglatin = w[1..].to_string() + "-";
                word_piglatin.push(ch);
                word_piglatin.push_str("ay");
            }
        }
        else { 
            word_piglatin = w.to_string();
        }
        return word_piglatin
    }
    "".to_string()
}

fn split_treat_char(it : &mut std::iter::Peekable<std::str::Chars<>>, 
    prev_is_alph : &mut bool) -> String {
    let mut word : String = String::new();
    loop {
        if let Some(ch) = it.peek() {
            if *prev_is_alph {
                if ch.is_alphabetic() {
                    *prev_is_alph = true;
                    word.push(*ch);
                }
                else{
                    *prev_is_alph = false;
                    return word;
                }
            }
            else{
                if ch.is_alphabetic() {
                    *prev_is_alph = true;
                    return word;
                }
                else{
                    *prev_is_alph = false;
                    word.push(*ch);
                }
            }
        }
        if let None = it.next() { break; }
    }
    word
}

fn split(s : &String) -> Vec<String> {
    let mut v : Vec<String>= Vec::new();
    let mut is_alph : bool;
    let mut it = s.chars().peekable();
    if let Some(ch) = it.peek() {
        if ch.is_alphabetic() { is_alph = true; }
        else { is_alph = false; }
        v.push( split_treat_char(&mut it, &mut is_alph) );

        while let Some(_) = it.peek() {
            v.push( split_treat_char(&mut it, &mut is_alph) );
        }
    }
    v
}

fn convert_to_piglatin2(s : &String) -> String {
    let mut piglatin = String::new();
    //let words = s.split( |ch| ch == ' ' || ch == ',' || ch == '.' ).filter(|s| *s != "" );
    let words = split(s);
    for w in words {
        piglatin.push_str( &convert_word_to_piglatin(&w) );
    }
    piglatin
}

fn main() {
    println!("Enter your message: ");
    let msg = readline();
    let pl1 = convert_to_piglatin1(&msg);
    let pl2 = convert_to_piglatin2(&msg);
    println!("Your message in pig latin #1: {}", pl1);
    println!("Your message in pig latin #2: {}", pl2);
    assert_eq!(pl1, pl2);
}
