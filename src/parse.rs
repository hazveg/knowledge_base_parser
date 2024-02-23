pub fn header(token: &str) -> String {
    let mut header_level = 0;
    let mut i = 0;
    let mut space_set = false;

    for char in token.chars() {
        if char == ' ' {
            space_set = true;
            break;
        }

        if i > 3 {
            break;
        }

        if char == '#' {
            header_level += 1;            
        }

        i += 1;
    }

    let token_content = match space_set {
        true => &token[header_level+1..],
        false => &token[header_level..],
    };

    format!("<h{}>{}</h{}>", header_level, token_content, header_level)
}

pub fn formatting(token: &str) -> String {
    let mut star_count = 0;

    for char in token.chars() {
        if char != '*' {
            break;
        }

        star_count += 1;
    }

    let token_content = &token[star_count..token.len()-star_count];

    match star_count {
        1 => format!("<i>{}</i>", token_content),
        2 => format!("<b>{}</b>", token_content),
        _ => "".to_string(),
    }
}

pub fn block_quote(token: &str) -> String {
    let mut space_set = false;
    let mut i = 0;

    for char in token.chars() {
        if char == ' ' {
            space_set = true;
            break;
        }

        if i > 0 {
            break;
        }

        i += 1;
    }

    let token_content = match space_set {
        true => &token[i+1..],
        false => &token[i..],
    };

    format!("<blockquote>{}</blockquote>", token_content)
}

pub fn image(token: &str) -> String {
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

    let alt = &token[alt_indices.0..alt_indices.1];
    let src = &token[src_indices.0..src_indices.1];

    format!("<img src=\"{}\" alt=\"{}\">", src, alt)
}

pub fn unordered_list(token: &str) -> String {
    let mut i = 0;
    let mut space_set = false;

    for char in token.chars() {
        if char == ' ' {
            space_set = true;
            break;
        }

        if i > 0 {
            break;
        }

        i += 1;
    }

    let token_content = match space_set {
        true => &token[2..],
        false => &token[1..],
    };

    format!("<li>{}</li>", token_content)
}
