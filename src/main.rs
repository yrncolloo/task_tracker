use std::{fs::File, io::{BufRead, BufReader, Read, Write}, path::Path};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TaskProperties{
    task_name: String,
    id: u32,
    mark: Marks
}

#[derive(Serialize, Deserialize, Debug)]
enum Marks {
    MarkInProgress,
    MarkDone,
    New

    
}

#[derive(Parser, Debug)]
#[command(version="0.1")]
#[command(about="Cli Task Tracker")]
#[command(propagate_version = true)]
struct Cli{
    #[arg(short, long, value_name="Item")]
    add: Option<String>,
    #[arg(short, long, value_name="Item ID")]
    update: Option<String>,
    #[arg(short, long, value_name="Item ID")]
    delete: Option<String>,
    #[arg(short, long, value_name="Items")]
    list: Option<String>

}
impl Cli {
    fn new() -> Self{
        let cli = Cli::parse();
        Self { 
            add: cli.add,
            update: cli.update,
            delete: cli.delete,
            list: cli.list 
        }
        
    }
    
    fn add_item(item:Cli) {
        let filename = "task.json";
        if !Path::new(filename).exists(){
            File::create(filename).expect("Sometthing went wrong");
            println!(" [*] created {}", filename);
        }
        let mut file = File::options()
            .append(true)
            .open(filename)
            .expect("Never opened the file Sometthing went wrong");
        let raw_items = item.add;

        let item_props = TaskProperties{
            id: 1,
            task_name: raw_items.expect("non"),
            mark: Marks::New,


        };
        id_generator(filename.to_owned());

        
        writeln!(&mut file, "{:?}", serde_json::to_string(&item_props).ok().unwrap()).expect("Could not write to file");
    }


}

fn id_generator(filename:String){

    let file_op = File::open(filename).expect("Something went wrong opening file");
    let mut read_buf = BufReader::new(&file_op);
    let mut file_contents = String::new();
    read_buf.read_to_string(&mut file_contents).expect("Something went wrong reading the file contents");


    let sp = file_contents.split("\n");

    let mut collect = sp.clone().collect::<Vec<&str>>();
    let final_len = collect.len().saturating_sub(2);
    collect.truncate(final_len);
    let last_item = collect.last();
    let bind = last_item.expect("could not open").to_string();
    let item:serde_json::Value = serde_json::from_str(&bind)
        .expect("something went wrong");

    dbg!(&item);
    let id = item.get("id");
    dbg!(id);






    // will continue from here folks
}

fn main() {
    let cli_init = Cli::new();
    Cli::add_item(cli_init);

}
