use lcs::wunsch::{diff, get_chars};

mod lcs;

fn main() {
    let lcs = diff(get_chars("HARBOUR"), get_chars("HABRAHABR"));
    println!("{:?}", lcs);
}


