
use std::collections::HashMap;
use std::string::String;

use std::fs;

fn index_html_files() -> HashMap<String, String> {
    //read the contents of the directory
    let paths = fs::read_dir("./public").unwrap();
    //create a hashmap to store the file names and their contents
    let mut files = HashMap::new();
    //iterate over the paths
    for path in paths {
        //unwrap the path
        let path = path.unwrap().path();
        //get the file name
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        //read the contents of the file
        let contents = fs::read_to_string(path).unwrap();
        //insert the file name and contents into the hashmap
        files.insert(file_name, contents);
    }
    //return the hashmap
    files

}