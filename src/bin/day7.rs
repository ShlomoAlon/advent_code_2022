use id_tree::Tree;
use advent_code_2022::read_file::read_day;
use crate::FileNode::{File, Folder};


enum FileNode{
    File{name: String, parent: usize, size: i64},
    Folder{name: String, children: Vec<usize>, parent: usize}
}
impl FileNode{
    fn add_child(& mut self, child: usize){
        match self {
            File { .. } => {panic!()}
            FileNode::Folder {name, children, parent } => {children.push(child)}
        }
    }
    fn get_children(& self) -> Vec<usize>{
        match s { }
    }
}
struct FileSystem{
    files: Vec<FileNode>
}

impl FileSystem{
    fn add_file(& mut self, name: String, parent: usize, size: i64) -> usize{
        self.files.push(FileNode::File {name, parent, size});
        let child = self.files.len() - 1;
        self.files[parent].add_child(child);
        child
    }
    fn add_folder(& mut self, name: String, parent: usize) -> usize{
        self.files.push(FileNode::Folder {name, children: Vec::new(), parent});
        let child = self.files.len() - 1;
        self.files[parent].add_child(child);
        child
    }
    fn new() -> Self{
        FileSystem{
            files: vec![Folder {name: "root".to_string(), children: Vec::new(), parent: 0}],
        }
    }
    fn get_child_by_name(& self, cur_folder: usize, name: String) -> usize{
        for child in self.files[cur_folder]
    }
}

fn is_cd(line: &str) -> Option<String>{
    if line.starts_with("$ cd"){
        Some(line.chars().skip(5).collect::<String>())
    } else {
        None
    }
}



fn main() {
    let mut file = read_day(7);
    let mut file = file.lines();
    let mut file_system = FileSystem::new();
    let mut cur: usize = 0;
    while let Some(line) = file.next(){
        if line.starts_with("$"){
            if let Some(go_to) = is_cd(line){

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cd(){
        let line = "$ cd sjt";
        assert_eq!(is_cd(line), Some("sjt".to_string()))
    }
}
