use aws_config::meta::region::RegionProviderChain;
use aws_sdk_imagebuilder::{Client, Error};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(required = true)]
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&config);

    let r = client
        .get_image()
        .set_image_build_version_arn(cli.name)
        .send()
        .await?;

    println!(
        "{}",
        r.image
            .unwrap()
            .output_resources
            .unwrap()
            .amis
            .unwrap()
            .first()
            .unwrap()
            .image
            .as_ref()
            .unwrap()
    );

    Ok(())
}
