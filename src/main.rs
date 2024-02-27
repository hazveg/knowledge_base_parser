use std::fs::File;
use std::io::{Read, Write};
use parse::{HtmlToken, TokenVariant};

mod parse;

fn create_file(title: &str, body: String) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.html", title))?;

    let html_string = format!("
<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <link rel=\"stylesheet\" href=\"styles.css\">
    <title>{}</title>
</head>
<body>
{}
</body>
</html>
", title, body);

    file.write_all(html_string.as_bytes())?;

    Ok(())
}

#[derive(Debug)]
struct Article {
    source_contents: String,
    images: Vec<String>,
}

impl Article {
    fn new(path: &str) -> std::io::Result<Self> {
        let source_contents = get_source_contents(path)?;
        let images = get_images(path)?;
        
        Ok(Self { source_contents, images })
    }

    fn to_html_tokens(&self) -> Result<Vec<HtmlToken>, parse::ParseError> {
        let mut tags = vec![];

        for token in self.source_contents.lines() {
            if token == "" {
                tags.push(HtmlToken { token: "<br>".to_string(), variant: TokenVariant::Break });
                continue;
            }

            let first_char = match token.chars().nth(0) {
                Some(char) => char,
                None => return Err(parse::ParseError::new(format!("Unable to grab first character from `{}`", token))),
            };

            let tag = match first_char {
                '#' => HtmlToken::header(token),
                '*' => HtmlToken::formatting(token),
                '>' => HtmlToken::block_quote(token),
                '!' => HtmlToken::image(&self.images, token),
                '-' => HtmlToken::unordered_list(token),
                char => {
                    if char.is_numeric() {
                        HtmlToken::ordered_list(token)
                    } else {
                        HtmlToken::paragraph(token)
                    }
                },
            };

            tags.push(tag);
        }

        Ok(tags)
    }
}

fn get_source_contents(path: &str) -> std::io::Result<String> {
    let mut file = File::open(format!("{}/src.md", path))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_images(path: &str) -> std::io::Result<Vec<String>> {
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
            Some(path) => {
                images.push(path.to_string());
            }
            None => {}
        }
    }

    Ok(images)
}

fn hydrate_tags(tags: &mut Vec<HtmlToken>) {
    let mut ordered_tag_indices = vec![];
    let mut unordered_tag_indices = vec![];
    for i in 0..tags.len() {
        match tags[i].variant {
            TokenVariant::OrderedList => ordered_tag_indices.push(i),
            TokenVariant::UnorderedList => unordered_tag_indices.push(i),
            _ => {},
        }
    }

    if !ordered_tag_indices.is_empty() {
        ordered_tag_indices.sort();

        let first_ordered_index = ordered_tag_indices[0];
        let last_ordered_index = ordered_tag_indices[ordered_tag_indices.len()-1];

        tags.insert(first_ordered_index, HtmlToken { token: "<ol type=\"1\">".to_string(), variant: TokenVariant::OrderedListDecorator });
        tags.insert(last_ordered_index+2, HtmlToken { token: "</ol>".to_string(), variant: TokenVariant::OrderedListDecorator });
    }

    if !unordered_tag_indices.is_empty() {
        unordered_tag_indices.sort();

        let first_unordered_index = unordered_tag_indices[0];
        let last_unordered_index = unordered_tag_indices[unordered_tag_indices.len()-1];

        tags.insert(first_unordered_index, HtmlToken { token: "<ul>".to_string(), variant: TokenVariant::UnorderedListDecorator });
        tags.insert(last_unordered_index+2, HtmlToken { token: "</ul>".to_string(), variant: TokenVariant::UnorderedListDecorator });
    }
}

fn generate_body(article: Article) -> Result<String, parse::ParseError> {
    let mut tags = article.to_html_tokens()?;
    hydrate_tags(&mut tags);

    let mut body = String::new();

    for tag in tags {
        let mut insert = String::new();

        insert.push_str(format!("{}\n", tag.token).as_str());
        body.push_str(&insert);
    }

    Ok(body)
}

fn main() {
    let dir = "mediothek_printer";

    let article = match Article::new(dir) {
        Ok(article) => article,
        Err(e) => {
            eprintln!("parser: {}", e);
            std::process::exit(1);
        }
    };

    let body = match generate_body(article) {
        Ok(body) => body,
        Err(e) => {
            eprintln!("parser: {}", e);
            std::process::exit(1);
        },
    };

    match create_file(dir, body) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("parser: {e}");
            std::process::exit(1);
        },
    }
}
