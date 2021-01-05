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

fn entry_to_str(ent: &fs::DirEntry) -> String {
    let ent_str = String::from(ent.path().file_name().unwrap().to_str().unwrap());
    ent_str
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

fn file_name_crypto(f_name: &String) -> String {
    let cry_name = chunk_encode(f_name);
    fs::rename(f_name, &cry_name).unwrap();
    cry_name
}

fn file_name_decrypto(f_name: &String) -> String {
    let dec_name = String::from_utf8(chunk_decode(f_name)).unwrap();
    fs::rename(f_name, &dec_name).unwrap();
    dec_name
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
                fs::remove_file(entry_to_str(&entry)).unwrap();
                println!(
                    "[-]{} was removed.",
                    entry_to_str(&entry)
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
    user$ file_crypto_base64 -cn  #Crypto current folders all files name.\n
    user$ file_crypto_base64 -dn  #Decrypto current folders all files name.\n
    Note: square brackets was files list it doesnt contain thire self.\n";

    let arg_flg_loc: usize = 1;
    let mut file_index: u32 = 1;
    let mut list_file_count: u32 = 1;
    let operation_flg = env::args().collect::<Vec<String>>();
    if operation_flg.len() > 1 {
        let current_path = Path::new(".");
        if operation_flg[arg_flg_loc].as_str() == "-c" {
            for file_name in &operation_flg[2..] {
                creat_crypto_file(&file_name);
                println!(
                    "[+]{} files cryptod, current file is {}",
                    file_index, file_name
                );
                file_index += 1
            }
        } else if operation_flg[arg_flg_loc].as_str() == "-d" {
            for file_name in &operation_flg[2..] {
                creat_decrypto_file(&file_name);
                println!(
                    "[+]{} files decryptod, current file is {}",
                    file_index, file_name
                );
                file_index += 1;
            }
        } else if operation_flg[arg_flg_loc].as_str() == "-lc" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if !entry_self_check(&entry) && !entry_contians(&entry, "%^%") {
                        let out_name = file_name_reorganization(
                            entry_to_str(&entry).as_str(),
                            "Cryptod",
                        );
                        creat_crypto_file(entry_to_str(&entry).as_str());
                        println!(
                            "[+]{} files cryptod, current file is {}",
                            &list_file_count, &out_name
                        );
                    }
                }
                list_file_count += 1;
            }
        } else if operation_flg[arg_flg_loc].as_str() == "-ld" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if !entry_self_check(&entry) {
                        let out_name = file_name_reorganization(
                            entry_to_str(&entry).as_str(),
                            "Deryptod",
                        );
                        creat_decrypto_file(entry_to_str(&entry).as_str());
                        println!(
                            "[+]{} files decryptod, current file is {}",
                            &list_file_count, &out_name
                        );
                    }
                }
                list_file_count += 1;
            }
        } else if operation_flg[arg_flg_loc].as_str() == "-cn" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if !entry_self_check(&entry) {
                        let cryptoed_name = file_name_crypto(&entry_to_str(&entry));
                        println!(
                            "[#]{} crytpod to {}",
                            &entry_to_str(&entry), &cryptoed_name
                        );
                    }
                }
            }
        } else if operation_flg[arg_flg_loc].as_str() == "-dn" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if !entry_self_check(&entry) {
                        let mata_name = file_name_decrypto(&entry_to_str(&entry));
                        println!(
                            "[#]{} decryptod to {}",
                            &entry_to_str(&entry), &mata_name
                        );
                    }
                }
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
