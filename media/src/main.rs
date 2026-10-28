#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        //     if let Media::Book { title, author } = self {
        //         format!("Book: '{}' by {}", title, author)
        //     } else if let Media::Movie { title, director } = self {
        //         format!("Movie: '{}' directed by {}", title, director)
        //     } else if let Media::Audiobook { title } = self {
        //         format!("Audiobook: '{}'", title)
        //     } else {
        //         String::from("Media description")
        //     }
        // }

        // pattern matching is more idiomatic than if let chains
        // if else chains are more useful for error handling, but pattern matching is more idiomatic for enums
        match self {
            Media::Book { title, author } => {
                format!("Book: '{}' by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: '{}' directed by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: '{}'", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: '{}'", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
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

    match catalog.items.get(0) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Item not found")
        }
    }
}
