struct EssayInfo<'a> {
    title : &'a str,
    author : &'a str,
}


fn main() {
    let essay = String::from(
        "My Cool Story\n\
        By Kevin\n\
        \n\
        Once upon a time there was a great wizard...");

    let mut lines = essay.split("\n");

    let title = lines.next()
                     .expect("Couldn't get a title line!");

    let author = lines.next()
                      .expect("Couldn't get an author line!")
                      .split(" ").skip(1).next()  // Skip "by" and take the author
                      .expect("Couldn't get an author!");

    let info = EssayInfo{title, author};

    println!("'{}' written by {}", info.title, info.author);
}
