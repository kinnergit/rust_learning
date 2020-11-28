mod io;
mod machine;

use crate::io::file::file_get_contents_with_buffer;

fn main() {
    let path = "src/main.rs";

    match file_get_contents_with_buffer(path) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line.unwrap());
            }
        }

        Err(e) => println!("error: {}", e),
    }
}
