use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

fn main() {
    let up_te_fn = "pip_up_li.tmp";
    let cmd_list = ["-c", "pip", "list", "-o", ">", &up_te_fn];
    Command::new("sh")
        .args(&cmd_list)
        .output()
        .expect("Command error.");
    let tmp_up_file_path = Path::new(&up_te_fn);
    let list_file_name = "pip_update.list";
    let list_file_path = Path::new(list_file_name);
    if tmp_up_file_path.is_file() {
        let tmp_file = fs::File::open(tmp_up_file_path).expect("Tmp file open failed.");
        let tmp_buff = BufReader::new(tmp_file);
        let mut line_num = 0;
        if list_file_path.is_file() {
            let mut out_list_fi = fs::File::open(list_file_path).expect("List file open failed.");
            for line in tmp_buff.lines() {
                if line_num > 1 {
                    let sp_tm_pkg_name = line.unwrap();
                    let sp_tm_na: Vec<&str> = sp_tm_pkg_name.split(" ").collect();
                    let sp_pkg_name = sp_tm_na[0];
                    if sp_pkg_name == "pip"{
                        Command::new("sh")
                            .args(&["-c", "python", "-m", "pip", "install", "--upgrade", "pip"])
                            .output()
                            .expect("Command error.");
                    }
                    let w_name = String::from(sp_pkg_name) + "\n";
                    out_list_fi
                        .write(w_name.as_bytes())
                        .expect("Tmp file write error.");
                    println!("{}", w_name);
                    let cmd_update = ["-c", "pip", "install", "--upgrade", "-r", &list_file_name];
                    Command::new("sh")
                        .args(&cmd_update)
                        .output()
                        .expect("Command error.");
                } else {
                    //
                }
                line_num += 1;
            }
        } else {
            let mut out_list_fi =
                fs::File::create(list_file_path).expect("List file create failed.");
            for line in tmp_buff.lines() {
                if line_num > 1 {
                    let sp_tm_pkg_name = line.unwrap();
                    let sp_tm_na: Vec<&str> = sp_tm_pkg_name.split(" ").collect();
                    let sp_pkg_name = sp_tm_na[0];
                    let w_name = String::from(sp_pkg_name) + "\n";
                    out_list_fi
                        .write(w_name.as_bytes())
                        .expect("Tmp file write error.");
                    println!("{}", w_name);
                } else {
                    //
                }
                line_num += 1;
            }
            let cmd_update = ["-c", "pip", "install", "--upgrade", "-r", &list_file_name];
            Command::new("sh")
                .args(&cmd_update)
                .output()
                .expect("Command error.");
        }
    } else {
        let tmp_file = fs::File::create(tmp_up_file_path).expect("Tmp file create failed.");
        let tmp_buff = BufReader::new(tmp_file);
        let mut line_num = 0;
        let mut out_list_fi =
            fs::File::create(list_file_path).expect("List file create failed.");
        for line in tmp_buff.lines() {
            if line_num > 1 {
                let sp_tm_pkg_name = line.unwrap();
                let sp_tm_na: Vec<&str> = sp_tm_pkg_name.split(" ").collect();
                let sp_pkg_name = sp_tm_na[0];
                let w_name = String::from(sp_pkg_name) + "\n";
                out_list_fi
                    .write(w_name.as_bytes())
                    .expect("Tmp file write error.");
                println!("{}", w_name);
            } else {
                //
            }
            line_num += 1;
        }
        let cmd_update = ["-c", "pip", "install", "--upgrade", "-r", &list_file_name];
        Command::new("sh")
            .args(&cmd_update)
            .output()
            .expect("Command error.");
    }
    fs::remove_file("pip_up_li.tmp").expect("Tmp remove error.");
    fs::remove_file("pip_update.list").expect("List remove error.");
    
}
