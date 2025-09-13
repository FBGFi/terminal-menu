mod print_menu_header;

pub struct TerminalMenuOptions<'a> {
    header_text: &'a str,
}

impl TerminalMenuOptions<'_> {
    pub fn new(header_text: &str) -> TerminalMenuOptions<'_> {
        TerminalMenuOptions {
            header_text,
        }
    }
}

pub fn run_terminal_menu(options: TerminalMenuOptions) {
    print_menu_header::print(options.header_text);
}
