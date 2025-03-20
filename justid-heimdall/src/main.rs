#[tokio::main]
async fn main() {
    // NOTE: Should I bring the configuration right before the service start and pass it to the
    // service or should I load the config inside the service. Think about this later on
    if let Err(e) = justid_heimdall::start_heimdall_service() {}
}
