
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices(sentence : &String) {
    let sl1 = &sentence[0..5];
    println!("sl1: {} : sl1: {}", sl1, sl1.len());
}

fn first_word_slice(s: &str) -> &str 
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() 
{
    let sentence = String::from("Guten Morgen!");
    let  fw = first_word(&sentence);
    println!("First word index: {}", fw);
    string_slices(&sentence);
    let slice = first_word_slice(&sentence[..]);
    
    println!("First word: {}", slice);
    
    let mut mrs : &str = "slice";
    println!("{}", &mrs);
    mrs = "hi!";
    println!("{}", &mrs);
}
