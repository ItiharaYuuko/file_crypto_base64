# File crypto base64

### Author: ItiharaYuuko
### E-Mail: wert3714@yahoo.com

This application using for crypto files, use base64 crate to crypto files.

At first you should to install the Rust compiler.
If you dont know how to install it, you should searching the rust-lang main
page at <https://www.rust-lang.org/tools/install>. To learn how install the rust compiler.

Next step, you should cloning the code from my github address.
If you installed the git client, you can type the command in console:
git clone <https://github.com/ItiharaYuuko/file_crypto_base64.git>  

When the operation has done. You can change the path to the project.  

### And executing commands:

```Shell
    user$ cargo clean #Clean objects target files.  
    user$ cargo build --release #Build binary for release mode.  
```

You could able to find out the executable binary file, in the current folder,
that under the target/release folder, the name same as the project.  

Before using the application, you should move the binnary file to your destination folder.  

Now change current path, to your destiation folder.  

## Command line usage:

```Shell
    user$ file_crypto_base64 -c [file names separated by blank]  #Crypto selected files.
    user$ file_crypto_base64 -d [file names separated by blank]  #Decrypto selected files.
    user$ file_crypto_base64 -lc  #Crypto current folders all files.
    user$ file_crypto_base64 -ld  #Decrypto current folders all files.
    user$ file_crypto_base64 -pm  #Remove all meta files.
    user$ file_crypto_base64 -pc  #Remove all cryptod files.
    user$ file_crypto_base64 -cn  #Crypto current folders all files name.
    user$ file_crypto_base64 -dn  #Decrypto current folders all files name.  
```

Note: square brackets was files list it doesnt contain thire self.  
Enjoy.
