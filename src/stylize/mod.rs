use colored::{ ColoredString, Styles };

pub fn add_styles(mut text: ColoredString, styles: Vec<Styles>) -> ColoredString {
    styles.iter().for_each(|style| {
        text.style.add(style.clone());
    });
    return text;
}
