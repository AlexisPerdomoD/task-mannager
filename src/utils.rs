use std::{path::PathBuf};

use home::home_dir;

pub fn set_default_route() -> Option<PathBuf>{
    return home_dir().map(| mut path |{
        path.push(".to-do-tasks.json");
        return path
    })
}