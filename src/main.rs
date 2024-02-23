use std::fs::File;
use std::io::{Read, Write};

mod parse;

struct FormattedInput {
    title: String,
    body: String,
}

fn create_file(formatted_input: FormattedInput) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.html", formatted_input.title))?;

    let html_string = format!("
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>{}</title>
</head>
<body>
    {}
</body>
</html>
", formatted_input.title, formatted_input.body);

    file.write_all(html_string.as_bytes())?;

    Ok(())
}

fn format_input(file_path: &str) -> Result<FormattedInput, String> {
    Ok(FormattedInput {
        title: "test".to_string(),
        body: "hello everybody my name is markiplier and welcome to five nights at freddy's".to_string(),
    })
}

fn parse_md(token: &str) -> Option<String> {
    let first_char = token.chars().nth(0)?;
    match first_char {
        '#' => Some(parse::header(token)),
        '*' => Some(parse::formatting(token)),
        '>' => Some(parse::block_quote(token)),
        '!' => Some(parse::image(token)),
        '-' => Some(parse::unordered_list(token)),
        _ => None,
    }
}

#[derive(Debug)]
struct Article {
    source_contents: String,
    images: Vec<String>,
}

impl Article {
    fn new(path: &str) -> Result<Self, std::io::Error> {
        let source_contents = get_source_contents(path)?;
        let images = get_images(path)?;
        
        Ok(Self { source_contents, images })
    }
}

fn get_source_contents(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(format!("{}/src.md", path))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_images(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut images = vec![];
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        let file_name = match entry.file_name().to_str() {
            Some(file_name) => file_name.to_string(),
            None => continue,
        };

        let file_name = file_name.to_string();

        if !(&file_name[file_name.len()-4..] == ".png") {
            continue;
        }

        match entry.path().to_str() {
            Some(path) => images.push(path.to_string()),
            None => {}
        }
    }

    Ok(images)
}

fn generate_body(article: Article) -> String {
    let mut tags = vec![];
    for line in article.source_contents.lines() {
        match parse_md(line) {
            Some(content) => tags.push(content),
            None => {},
        }
    }

    let mut body = String::new();

    for tag in tags {
        body.push_str(&tag);
    }

    body
}

fn main() {
    /*let formatted_input = match format_input("mediothek_printer") {
        Ok(formatted_input) => formatted_input,
        Err(e) => {
            eprintln!("something happened: {e}");
            std::process::exit(1);
        },
    };
    match create_file(formatted_input) {
        Ok(_) => {},
        Err(e) => eprintln!("something happened: {e}"),
    }*/
    let article = match Article::new("mediothek_printer") {
        Ok(article) => article,
        Err(e) => {
            eprintln!("something happened: {}", e);
            std::process::exit(1);
        }
    };

    dbg!(&article);

    println!("{}", generate_body(article));
}
