use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "\n
Usage:\n
    akv_mem.exe <FILE> get <KEY>\n
    akv_mem.exe <FILE> delete <KEY> <VALUE>\n
    akv_mem.exe <FILE> insert <KEY> <VALUE>\n
    akv_mem.exe <FILE> update <KEY> <VALUE>\n
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "\n
Usage:\n
    akv_mem <FILE> get <KEY>\n
    akv_mem <FILE> delete <KEY> <VALUE>\n
    akv_mem <FILE> insert <KEY> <VALUE>\n
    akv_mem <FILE> update <KEY> <VALUE>\n
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f_name =  args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&f_name);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value),
        },

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap()
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap()
        }

        _ => eprintln!("{}", &USAGE),
    }
}