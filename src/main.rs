use std::{env, fs, path::PathBuf, process};

fn main() {
    let mut args = env::args();
    args.next();

    let path = args.next().unwrap_or_else(|| {
        eprintln!("No file or directory specified");
        process::exit(1);
    });

    let is_dir = fs::metadata(&path)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        })
        .is_dir();

    let mut line_count = 0;

    if is_dir {
        line_count = count_lines_in_dir(vec![PathBuf::from(path)], 0);
    } else {
        let file = fs::read_to_string(&path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

        for _ in file.lines() {
            line_count += 1;
        }
    }

    println!("{line_count} lines were counted");
}

// dir_paths = list of directories to check
// check every path in directory, read the ones that are files
// puth paths that are directories in new vec
// call path again with new paths list
// return line count when paths list is empty
fn count_lines_in_dir(dir_paths: Vec<PathBuf>, mut line_count: i32) -> i32 {
    let mut new_dir_paths = Vec::new();

    for path in &dir_paths {
        let inner_paths = fs::read_dir(path)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect::<Vec<_>>();

        for path in inner_paths {
            let is_dir = fs::metadata(&path)
                .unwrap_or_else(|err| {
                    eprintln!("{err}");
                    process::exit(1);
                })
                .is_dir();

            if is_dir {
                new_dir_paths.push(path.clone());
            } else {
                let file = fs::read_to_string(&path).unwrap_or_else(|err| {
                    eprintln!("{err}");
                    process::exit(1);
                });

                for _ in file.lines() {
                    line_count += 1;
                }
            }
        }
    }

    // This is a real "I have no idea how this works" moment
    if !dir_paths.is_empty() {
        count_lines_in_dir(new_dir_paths, line_count)
    } else {
        line_count
    }
}
