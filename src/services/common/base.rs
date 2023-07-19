type TFileDescriptorSet = &'static [u8];

pub struct GrpcServerConfig {
    pub file_descriptor_set: TFileDescriptorSet,
}

impl GrpcServerConfig {
    pub fn new(params: GrpcServerConfigNewParam) -> Self {
        Self {
            file_descriptor_set: params.file_descriptor_set,
        }
    }
    pub fn get_descriptor(&self) -> TFileDescriptorSet {
        self.file_descriptor_set
    }
}

pub struct GrpcServerConfigNewParam {
    pub file_descriptor_set: TFileDescriptorSet,
}
