fn main() {
    let protos_path = "proto";
    let file_descriptor_path = "blabla";

    //Proto files will be compiled here & also reflection feature will be initialized here
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path("ReflectionFilePath")
        .compile(&["proto/user/v1/user.proto"], &[protos_path])
        .expect("Error on proto files build");
}
