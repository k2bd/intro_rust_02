struct EssayInfo<'a> {
    title : &'a str,
    author : &'a str,
}


fn main() {
    let info;
    {
        let essay = String::from(
            "My Great Story\n\
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

        info = EssayInfo{title, author};
    }
    println!("'{}' written by {}", info.title, info.author);
}
