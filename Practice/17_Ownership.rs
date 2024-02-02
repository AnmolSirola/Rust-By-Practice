fn main(){

    let s1 = "hello";
    let s2 = s1;
    println!(" {:?} {:?}", s1, s2);

    let s1_owned = String::from("Hello");
    let s2_owned = s1_owned;
    println!("{:?} {:?}", s1_owned, s2_owned);
}

-------------------------------------------------

struck Book {
    pages: i32,
    ratings: i32,
}

fn displays_page_count(book: &book){   //using & to point as a refernce
    println!("pages={:?}", book.pages)
}

fn displays_page_ratings(book: &book){
    println!("pages={:?}", book.ratings)
}

fn main(){
    let book = Book{
        pages: 5,
        ratings = 9,
    };
    displays_page_count(&book);
    displays_page_ratings(&book);

}