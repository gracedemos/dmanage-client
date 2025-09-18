use crate::dmanage::{vm_manage_client::VmManageClient, VmManageRequest};

mod dmanage {
    tonic::include_proto!("dmanage");
}

const ID: u32 = 0;
const KEY: &str = "REPLACE_WITH_KEY";

#[unsafe(no_mangle)]
pub extern "C" fn unmap_gpu() -> i32 {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async move {
        let mut client = VmManageClient::connect("grpc://192.168.68.64:8080").await?;

        let request = tonic::Request::new(VmManageRequest {
            id: ID,
            key: KEY.to_string(),
        });

        client.unmap_gpu(request).await?;

        Ok(())
    });

    match result {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn remap_gpu() -> i32 {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async move {
        let mut client = VmManageClient::connect("grpc://192.168.68.64:8080").await?;

        let request = tonic::Request::new(VmManageRequest {
            id: ID,
            key: KEY.to_string(),
        });

        client.remap_gpu(request).await?;

        Ok(())
    });

    match result {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn start_win11() -> i32 {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async move {
        let mut client = VmManageClient::connect("grpc://192.168.68.64:8080").await?;

        let request = tonic::Request::new(VmManageRequest {
            id: ID,
            key: KEY.to_string(),
        });

        client.start_win11(request).await?;

        Ok(())
    });

    match result {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn stop_win11() -> i32 {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result: Result<(), Box<dyn std::error::Error>> = runtime.block_on(async move {
        let mut client = VmManageClient::connect("grpc://192.168.68.64:8080").await?;

        let request = tonic::Request::new(VmManageRequest {
            id: ID,
            key: KEY.to_string(),
        });

        client.stop_win11(request).await?;

        Ok(())
    });

    match result {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unmap_gpu() {
        assert_eq!(unmap_gpu(), 0);
    }

    #[test]
    fn test_remap_gpu() {
        assert_eq!(remap_gpu(), 0);
    }
}
