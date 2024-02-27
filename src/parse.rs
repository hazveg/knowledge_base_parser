pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An Error occured during parsing: {}", self.message)
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenVariant {
    Paragraph,
    Header,
    Break,
    Formatting,
    BlockQuote,
    Image,
    UnorderedList,
    UnorderedListDecorator,
    OrderedList,
    OrderedListDecorator,
}

#[derive(Debug)]
pub struct HtmlToken {
    pub token: String,
    pub variant: TokenVariant,
}

impl HtmlToken {
    pub fn paragraph(token: &str) -> Self {
        Self {
            token: token.to_string(),
            variant: TokenVariant::Paragraph,
        }
    }

    pub fn header(token: &str) -> Self {
        let mut header_level = 0;

        for char in token.chars() {
            if char == '#' {
                header_level += 1;            
            }
        }

        Self {
            token: format!("<h{}>{}</h{}>", header_level, &token[header_level+1..], header_level),
            variant: TokenVariant::Header,
        }
    }

    pub fn formatting(token: &str) -> Self {
        let mut star_count = 0;

        for char in token.chars() {
            if char != '*' {
                break;
            }

            star_count += 1;
        }

        let token_content = &token[star_count..token.len()-star_count];

        let token = match star_count {
            1 => format!("<i>{}</i>", token_content),
            2 => format!("<b>{}</b>", token_content),
            _ => "".to_string(),
        };

        Self {
            token,
            variant: TokenVariant::Formatting,
        }
    }

    pub fn block_quote(token: &str) -> Self {
        Self {
            token: format!("<blockquote>{}</blockquote>", &token[1..]),
            variant: TokenVariant::BlockQuote,
        }
    }

    pub fn image(images: &Vec<String>, token: &str) -> Self {
        let mut alt_indices = (0, 0);
        let mut src_indices = (0, 0);
        let mut i = 0;
        
        for char in token.chars() {
            if char == '[' {
                alt_indices.0 = i+1;
            }

            if char == ']' {
                alt_indices.1 = i;
            }

            if char == '(' {
                src_indices.0 = i+1
            }

            if char == ')' {
                src_indices.1 = i;
            }
            
            i += 1;
        }

        let token_chars: Vec<char> = token.chars().collect();

        let alt = &token_chars[alt_indices.0..alt_indices.1];
        let src = &token_chars[src_indices.0..src_indices.1];

        let mut alt_string = String::new();
        let mut src_string = String::new();

        for char in alt {
            alt_string.push(*char);
        }

        for char in src {
            src_string.push(*char);
        }

        for image in images {
            if !image.contains(&src_string) {
                continue;
            }

            src_string = image.to_string();
        }

        Self {
            token: format!("<img src=\"{}\" alt=\"{}\">", src_string, alt_string),
            variant: TokenVariant::Image,
        }
    }

    pub fn unordered_list(token: &str) -> Self {
        Self {
            token: format!("<li>{}</li>", &token[2..]),
            variant: TokenVariant::UnorderedList,
        }
    }

    pub fn ordered_list(token: &str) -> Self {
        let mut i = 0;
        let mut point_index = 0;

        for char in token.chars() {
            if char == '.' {
                point_index = i+1;
            }

            i += 1;
        }

        Self {
            token: format!("<li>{}</li>", &token[point_index+1..]),
            variant: TokenVariant::OrderedList,
        }
    }
}


