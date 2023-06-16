pub(crate) mod input {
    use std::env;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn accept_args() -> Vec<String> {
        return env::args().collect::<Vec<String>>();
    }

    pub fn read_file(arg: &String) -> String {
        let filename = arg;
        let mut f: File = File::open(filename).expect("file not found!");
        let mut contents: String = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading the file contents...");

        return contents;
    }

    pub fn initial_load(arg: Vec<&str>) -> (Vec<Vec<&str>>, HashMap<String, i32>) {
        let mut pointer: usize = 0;
        let mut program_counter: Vec<Vec<&str>> = Vec::new();
        let mut label_counter: HashMap<String, i32> = HashMap::new();

        for line in arg {
            let tmp = line.split_whitespace().collect::<Vec<&str>>();
            if tmp[0] == "label" {
                println!("Labeled here! l.{}", pointer);
                label_counter.insert(tmp[1].to_owned(), pointer.try_into().unwrap());
            }
            program_counter.push(tmp);
            pointer += 1;
        }

        return (program_counter, label_counter);
    }
}
