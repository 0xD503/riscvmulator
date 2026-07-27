pub fn print_license_text() {
    static LICENSE_TEXT: &str = "

    Copyright 2026 Ruslan Vostretsov

    Licensed under either of:

    * MIT License (LICENSE-MIT)
        https://opensource.org/licenses/MIT

    * Apache License, Version 2.0 (LICENSE-APACHE)
        https://www.apache.org/licenses/LICENSE-2.0

    at your option.

";
    print!("{LICENSE_TEXT}");
}
