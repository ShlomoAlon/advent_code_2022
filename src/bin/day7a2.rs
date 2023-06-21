use std::rc::Rc;
use advent_code_2022::read_file::read_day;
use crate::Node::FolderNode;

struct Folder{
    name: String,
    children: Vec<Node>,
    parent: Option<FolderLink>,

}


enum Node{
    File{name: String, size: i64},
    FolderNode(FolderLink),
}
type FolderLink = Rc<Folder>;

fn main() {
    let mut file = read_day(7);
    let mut file = file.lines();
    let mut cur = Rc::new(Folder {name: "/".to_string(), children: Vec::new(), parent: None});
    let mut root = cur.clone();
    let cd_regex = regex::Regex::new(r"^\$ cd (.*)$").unwrap();
    let ls_regex = regex::Regex::new(r"^\$ ls$").unwrap();
    let ls_result_file_regex = regex::Regex::new(r"^(\d+) (.+)$").unwrap();
    let ls_result_folder_regex = regex::Regex::new(r"^dir (.+)$").unwrap();
    for line in file {
        if let Some(capture) = cd_regex.captures(line){
            let name = capture.get(1).unwrap().as_str();
            if name == ".."{
                cur = cur.parent.clone().unwrap().clone();
            } else {
                for child in cur.children.iter(){
                    if let Node::FolderNode(folder) = child{
                        if folder.name == name{
                            cur = folder.clone();
                            break;
                        }
                    }
                }
            }
        } else if let Some(capture) = ls_regex.captures(line){
            continue
        } else if let Some(capture) = ls_result_file_regex.captures(line){
            let size = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let name = capture.get(2).unwrap().as_str();
            cur.children.push(Node::File {name: name.to_string(), size});
        } else if let Some(capture) = ls_result_folder_regex.captures(line){
            let name = capture.get(1).unwrap().as_str();
            let folder = Rc::new(Folder {name: name.to_string(), children: Vec::new(), parent: Some(cur.clone())});
            cur.children.push(Node::FolderNode(folder.clone()));
        } else {
            panic!("Invalid line: {}", line);
        }
    }
}

