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
    let out_file_name = file_name_reorganization(f_name, "cryptod");
    fs::write(out_file_name.as_str(), &crypto_context).unwrap();
}

fn creat_decrypto_file(f_name: &str) {
    let data_vec = genfile_data(f_name);
    let crypto_context = chunk_decode(&data_vec);
    let out_file_name = file_name_reorganization(f_name, "decryptod");
    fs::write(out_file_name.as_str(), &crypto_context).unwrap();
}

fn file_name_reorganization(f_name: &str, bet_flg: &str) -> String {
    let f_name_pre = f_name.get_split_at(".", 0);
    let f_extension = f_name.get_split_at(".", 1);
    format!("{}_{}.{}", f_name_pre, bet_flg, f_extension)
}

fn major_progress() {
    let pleaseholder_information = "
    Wrong console argument after application.\n
    Please choise:\n
    user$ file_crypto_base64 -c [file names separated by blank]  #Crypto selected files.\n
    user$ file_crypto_base64 -d [file names separated by blank]  #Decrypto selected files.\n
    user$ file_crypto_base64 -lc  #Crypto current folders all files.\n
    user$ file_crypto_base64 -ld  #Decrypto current folders all files.\n
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
                        "{} files cryptod, current file is {}",
                        file_index,
                        file_name
                    )
                }
                file_index += 1;
            }
        } else if operation_flg[1].as_str() == "-d" {
            for file_name in env::args() {
                if file_index > 1 {
                    creat_decrypto_file(&file_name);
                    println!(
                        "{} files cryptod, current file is {}",
                        file_index,
                        file_name
                    )
                }
                file_index += 1;
            }
        } else if operation_flg[1].as_str() == "-lc" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if entry.path().file_name().unwrap().to_str().unwrap() != operation_flg[0] {
                        let out_name = file_name_reorganization(
                            &entry.path().file_name().unwrap().to_str().unwrap(),
                            "cryptod",
                        );
                        creat_crypto_file(&entry.path().file_name().unwrap().to_str().unwrap());
                        println!(
                            "{} files cryptod, current file is {}",
                            &list_file_count,
                            &out_name
                        );
                    }
                }
                list_file_count += 1;
            }
        } else if operation_flg[1].as_str() == "-ld" {
            for entry in current_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    if entry.path().file_name().unwrap().to_str().unwrap() != operation_flg[0] {
                        let out_name = file_name_reorganization(
                            &entry.path().file_name().unwrap().to_str().unwrap(),
                            "decryptod",
                        );
                        creat_decrypto_file(&entry.path().file_name().unwrap().to_str().unwrap());
                        println!(
                            "{} files decryptod, current file is {}",
                            &list_file_count,
                            &out_name
                        );
                    }
                }
                list_file_count += 1;
            }
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
