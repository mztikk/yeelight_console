use apyee::{device::Device, method::Method};
use clap::{Parser, Subcommand};
use std::net::IpAddr;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    device_ip: IpAddr,
    #[clap(short, long)]
    device_port: Option<u16>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Toggle,
    SetPower {
        #[clap(action = clap::ArgAction::Set)]
        power: bool,
    },
    SetRgb {
        red: u8,
        green: u8,
        blue: u8,
    },
    SetColor {
        color: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut device = match cli.device_port {
        Some(port) => {
            println!("Connecting to device at {}:{}", cli.device_ip, port);
            Device::new_with_port(cli.device_ip, port).await?
        }
        None => {
            println!("Connecting to device at {}", cli.device_ip);
            Device::new(cli.device_ip).await?
        }
    };

    println!("Connected to device on {}:{}", device.ip, device.port);

    // output commands
    match cli.command {
        Commands::Toggle => {
            println!("Toggle");
            device.toggle().await?;
        }
        Commands::SetPower { power } => {
            println!("Set power to {}", power);
            device
                .execute_method(Method::SetPower(power, None, None))
                .await?;
        }
        Commands::SetRgb { red, green, blue } => {
            println!("Set RGB to {} {} {}", red, green, blue);
            device.set_rgb(red, green, blue).await?;
        }
        Commands::SetColor { color } => {
            let parsed_color = csscolorparser::parse(&color)?;
            println!(
                "Set color to {} (parsed as rgb: {})",
                color,
                &parsed_color.to_rgb_string()
            );
            let rgba = parsed_color.rgba_u8();
            device.set_rgb(rgba.0, rgba.1, rgba.2).await?;
        }
    }

    Ok(())
}
