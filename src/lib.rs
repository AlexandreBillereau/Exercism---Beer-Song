pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n-1, if n - 1 > 1 {"bottles"} else {"bottle"}),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut start = start;
    let mut string_final: Vec<String> = vec![];

    while start > end {
        string_final.push(verse(start));
        string_final.push("\n".to_string());
        start -= 1;
    }
    string_final.push(verse(start));

    string_final.concat()
}
