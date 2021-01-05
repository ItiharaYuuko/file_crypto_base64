use base64::{decode, encode};
use std::convert::AsRef;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

trait SplitAt {
    fn get_split_at(self, sep: &str, index: usize) -> String;
}

impl SplitAt for String {
    fn get_split_at(self, sep: &str, index: usize) -> String {
        format!("{}", self.split(sep).collect::<Vec<&str>>()[index])
    }
}

impl SplitAt for &str {
    fn get_split_at(self, sep: &str, index: usize) -> String {
        format!("{}", self.split(sep).collect::<Vec<&str>>()[index])
    }
}

fn entry_self_check(ent: &fs::DirEntry) -> bool {
    let flg_pa: bool;
    let app_self: Vec<String> = env::args().collect();
    if ent
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains(&app_self[0])
    {
        flg_pa = true;
    } else {
        flg_pa = false;
    }
    flg_pa
}

fn entry_contians(ent: &fs::DirEntry, context: &str) -> bool {
    let flg_pa: bool;
    if ent
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains(context)
    {
        flg_pa = true;
    } else {
        flg_pa = false;
    }
    flg_pa
}

fn chunk_encode<T: Clone + AsRef<[u8]>>(input: T) -> String {
    encode(&input)
}

fn chunk_decode<T: Clone + AsRef<[u8]>>(input: T) -> Vec<u8> {
    decode(&input).unwrap()
}

fn genfile_data(f_name: &str) -> Vec<u8> {
    let insert_file = File::open(f_name).unwrap();
    let mut copyx = insert_file.try_clone().unwrap();
    let mut vexc: Vec<u8> = Vec::new();
    copyx.read_to_end(&mut vexc).unwrap();
    vexc
}

fn creat_crypto_file(f_name: &str) {
    let data_vec = genfile_data(&f_name);
    let crypto_context = chunk_encode(&data_vec);
    let out_file_name = file_name_reorganization(f_name, "Cryptod");
    fs::write(out_file_name.as_str(), &crypto_context).unwrap();
}

fn creat_decrypto_file(f_name: &str) {
    let data_vec = genfile_data(f_name);
    let crypto_context = chunk_decode(&data_vec);
    let out_file_name = file_name_reorganization(f_name, "Deryptod");
    fs::write(out_file_name.as_str(), &crypto_context).unwrap();
}

fn file_name_reorganization(f_name: &str, bet_flg: &str) -> String {
    let tmp_str: String;
    if f_name.contains("Cryptod") {
        tmp_str = format!("{}", f_name.get_split_at("%^%", 1));
    } else {
        tmp_str = format!("{}%^%{}", bet_flg, f_name);
    }
    tmp_str
}

fn purge_mata_file(purge_flag: bool) {
    let ctr_pat = Path::new(".");
    for entry in ctr_pat.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let mata_flag: bool;
            if purge_flag {
                mata_flag = !entry_contians(&entry, "%^%");
            } else {
                mata_flag = entry_contians(&entry, "%^%");
            }
            if mata_flag && !entry_self_check(&entry) {
                fs::remove_file(&entry.path().file_name().unwrap().to_str().unwrap()).unwrap();
                println!(
                    "[-]{} was removed.",
                    &entry.path().file_name().unwrap().to_str().unwrap()
                );
            }
        }
    }
}

fn major_progress() {
    let pleaseholder_information = "
    Wrong console argument after application.\n
    Please choise:\n
    user$ file_crypto_base64 -c [file names separated by blank]  #Crypto selected files.\n
    user$ file_crypto_base64 -d [file names separated by blank]  #Decrypto selected files.\n
    user$ file_crypto_base64 -lc  #Crypto current folders all files.\n
    user$ file_crypto_base64 -ld  #Decrypto current folders all files.\n
    user$ file_crypto_base64 -pm  #Remove all meta files.\n
    user$ file_crypto_base64 -pc  #Remove all cryptod files.\n
    Note: square brackets was files list it doesnt contain thire self.\n";

    let mut file_index: u32 = 0;
    let mut list_file_count: u32 = 1;
    let operation_flg: Vec<String> = env::args().collect();
    if operation_flg.len() > 1 {
        let current_path = Path::new(".");
        if operation_flg[1].as_str() == "-c" {
            for file_name in env::args() {
                if file_index > 1 {
                    creat_crypto_file(&file_name);
                    println!(
                        "[+]{} files cryptod, current file is {}",
                        file_index, file_name
                    );
                }
                file_index += 1;
            }
        } else if operation_flg[1].as_str() == "-d" {
            for file_name in env::args() {
                if file_index > 1 {
                    creat_decrypto_file(&file_name);
                    println!(
                        "[+]{} files decryptod, current file is {}",
                        file_index, file_name
                    );
                }
                file_index += 1;
            }
        } else if operation_flg[1].as_str() == "-lc" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if !entry_self_check(&entry) && !entry_contians(&entry, "%^%") {
                        let out_name = file_name_reorganization(
                            &entry.path().file_name().unwrap().to_str().unwrap(),
                            "Cryptod",
                        );
                        creat_crypto_file(&entry.path().file_name().unwrap().to_str().unwrap());
                        println!(
                            "[+]{} files cryptod, current file is {}",
                            &list_file_count, &out_name
                        );
                    }
                }
                list_file_count += 1;
            }
        } else if operation_flg[1].as_str() == "-ld" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if !entry_self_check(&entry) {
                        let out_name = file_name_reorganization(
                            &entry.path().file_name().unwrap().to_str().unwrap(),
                            "Deryptod",
                        );
                        creat_decrypto_file(&entry.path().file_name().unwrap().to_str().unwrap());
                        println!(
                            "[+]{} files decryptod, current file is {}",
                            &list_file_count, &out_name
                        );
                    }
                }
                list_file_count += 1;
            }
        } else if operation_flg[1].as_str() == "-pm" {
            purge_mata_file(true);
        } else if operation_flg[1].as_str() == "-pc" {
            purge_mata_file(false);
        } else {
            panic!("{}", &pleaseholder_information);
        }
    } else {
        panic!("{}", &pleaseholder_information);
    }
}

fn main() {
    major_progress();
}
