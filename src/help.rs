pub mod help {

    /// print help
    pub fn help() {

        println!("USAGE:");
        println!("\tprivate ACTION TARGET1 TARGET2 ...");

        println!("ACTIONS:");
        println!("\tencrypt.....encrypt targets");
        println!("\tdecrypt.....decrypt targets");
        println!("\thash.....print sha3 hash of targets");

        println!("EXAMPLES:");
        println!("\tprivate encrypt myfile.txt myfolder/subfolder");
        println!("\tprivate decrypt file.txt myimage.jpg myfolder/subfolder");
        println!("\tprivate hash myfile.txt myfolder/subfolder");
    }
}