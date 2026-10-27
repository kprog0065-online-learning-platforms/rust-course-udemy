#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: '{}' by {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: '{}' directed by {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: '{}'", title)
        } else {
            String::from("Media description")
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

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

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
}
