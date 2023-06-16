pub(crate) mod input {
    use std::fs::File;
    use std::io::prelude::*;

    pub fn fileio(arg: &String) -> String {
        let filename = arg;
        let mut f: File = File::open(filename).expect("file not found!");
        let mut contents: String = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading the file contents...");

        return contents;
    }
}
