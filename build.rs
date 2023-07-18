use std::{env, path::PathBuf};

fn main() {
    let protos_path = "proto";

    //This outdir file defined by cargo
    //If the package has a build script, this is set to the folder where the build script should place its output. See below for more information. (Only set during compilation.)
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let user_desciptor_path = out_dir.join("user_descriptor.bin");

    //Proto files will be compiled here & also reflection feature will be initialized here
    tonic_build::configure()
        .file_descriptor_set_path(user_desciptor_path)
        .compile(&["proto/user/v1/user.proto"], &[protos_path])
        .unwrap();
    // .expect("Error on proto files build");
}
