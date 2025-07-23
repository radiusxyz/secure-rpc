fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();
    secure_rpc_cli::run()?;
    Ok(())
}
