mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("A Good Movie"),
        director: String::from("A Great Director"),
    };
    let bad_book = Media::Book {
        title: String::from("A Bad Book"),
        author: String::from("A Terrible Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(3);
    let placeholder = Media::Placeholder;

    println!("{:#?}", item.unwrap_or(&placeholder));
}
