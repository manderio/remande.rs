use notify_rust::Notification;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

enum Item {
    Remander(Remander), // rename these
    ToDo(ToDo),         // rename these
}

// define remander datatype
struct Remander {
    title: String,
    contents: String,
    // time
}

struct ToDo {
    title: String,
    contents: String,
    remander: Option<Remander>,
}

struct NamedItemList {
    title: String,
    items: Vec<Item>,
}

struct Opt {
    // debug
    #[
    // debug mode
    #[structopt(long)]
    debug: bool, 

    // itemType
    #[structopt(short = "t", long = "type")]
    itemType: String,


}

fn create_todo(title: String, contents: String, remander: Option<Remander>) -> ToDo {
    ToDo {
        title,
        contents,
        remander,
    }
}

fn create_remander(title: String, contents: String) -> Remander {
    Remander { title, contents }
}

fn create_list(title: String, items: Vec<Item>) -> NamedItemList {
    NamedItemList { title, items }
}

//TODO: Create structOpt arguments
fn main() {
    let remander = create_remander(arg(), arg()); // switch arg() for structopt

    notif(&remander)
}

// yeet in favor of structopt
// fn arg() -> String {
//     let mut buffer = String::new(); // fix this and make it work please
//     io::stdin() // understand this and work on fixing this
//         .read_line(&mut buffer)
//         .expect("this doesn't work");
//     buffer
// }

fn notif(remander: &Remander) {
    Notification::new()
        .summary(&remander.title)
        .body(&remander.contents)
        .show()
        .expect("This didn't work :c");
}
