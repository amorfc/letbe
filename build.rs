use std::{env, path::PathBuf};

fn main() {
    let protos_path = "proto";

    //This outdir file defined by cargo
    //If the package has a build script, this is set to the folder where the build script should place its output. See below for more information. (Only set during compilation.)
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let file_descriptor_path = out_dir.join("lett_descriptor.bin");

    //Proto files will be compiled here & also reflection feature will be initialized here
    tonic_build::configure()
        .file_descriptor_set_path(file_descriptor_path)
        .compile(
            &["proto/user/v1/user.proto", "proto/club/v1/club.proto"],
            &[protos_path],
        )
        .unwrap();
    // .expect("Error on proto files build");
}
