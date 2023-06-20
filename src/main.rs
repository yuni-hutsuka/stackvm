mod execute;
mod input;

fn main() {
    // 引数受け取り
    let args: Vec<String> = input::input::accept_args();

    // ファイルオープン
    let contents: String = input::input::read_file(&args[1]);
    // contentsを分割
    let lines = contents.lines().collect::<Vec<&str>>();
    // initial_load
    let (program_counter, label_counter) = input::input::initial_load(lines);

    execute::execution::execution(program_counter, label_counter);
}
