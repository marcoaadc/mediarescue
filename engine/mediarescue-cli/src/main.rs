use clap::{Parser, Subcommand};
use mediarescue_core::device::image::DiskImage;
use mediarescue_core::device::reader::DeviceReader;
use mediarescue_core::pipeline::orchestrator::RecoveryOrchestrator;
use mediarescue_core::types::{ScanConfig, ScanDepth};
use std::path::PathBuf;
use tokio::sync::broadcast;

#[derive(Parser)]
#[command(name = "mediarescue", about = "Professional data recovery tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        #[arg(short, long, help = "Path to disk image file (.dd, .img)")]
        image: String,

        #[arg(short, long, default_value = "recovered", help = "Output directory")]
        output: String,

        #[arg(short, long, default_value = "standard", help = "Scan depth: quick, standard, deep")]
        depth: String,
    },
    Devices {
        #[arg(short, long, help = "List all available devices")]
        list: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("mediarescue=info")
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { image, output, depth } => {
            let depth = match depth.as_str() {
                "quick" => ScanDepth::Quick,
                "deep" => ScanDepth::Deep,
                _ => ScanDepth::Standard,
            };

            let config = ScanConfig {
                depth,
                ..Default::default()
            };

            let reader = DiskImage::open(&image)?;
            println!("Scanning: {} ({} sectors)", image, reader.total_sectors());

            let orchestrator = RecoveryOrchestrator::new(config, PathBuf::from(&output));
            let (event_tx, mut event_rx) = broadcast::channel(256);

            let handle = tokio::spawn(async move {
                orchestrator.run(Box::new(reader), event_tx).await
            });

            // Print progress events
            tokio::spawn(async move {
                while let Ok(event) = event_rx.recv().await {
                    match event {
                        mediarescue_core::types::RecoveryEvent::ScanProgress {
                            sectors_done,
                            sectors_total,
                            signatures_found,
                            speed_mbps,
                        } => {
                            let pct = (sectors_done as f64 / sectors_total as f64) * 100.0;
                            print!("\r[{:.1}%] Scanned {}/{} sectors | {} signatures | {:.1} MB/s",
                                pct, sectors_done, sectors_total, signatures_found, speed_mbps);
                        }
                        mediarescue_core::types::RecoveryEvent::FileRecovered { id, score } => {
                            println!("\n  Recovered: {} (score: {:.0}%)", &id.to_string()[..8], score * 100.0);
                        }
                        mediarescue_core::types::RecoveryEvent::ScanComplete {
                            total_found,
                            total_recovered,
                            duration_secs,
                        } => {
                            println!("\n\nScan complete in {:.1}s", duration_secs);
                            println!("  Found: {} files", total_found);
                            println!("  Recovered: {} files", total_recovered);
                            println!("  Output: {}", output);
                        }
                        _ => {}
                    }
                }
            });

            match handle.await? {
                Ok(session) => {
                    println!("\nSession saved: {}", session.id);
                }
                Err(e) => {
                    eprintln!("\nScan failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Devices { list: _ } => {
            use mediarescue_core::device::enumerator::{DeviceEnumerator, SystemDeviceEnumerator};
            let enumerator = SystemDeviceEnumerator::new();
            match enumerator.list_devices() {
                Ok(devices) => {
                    if devices.is_empty() {
                        println!("No devices found. Try running as administrator.");
                    } else {
                        for d in &devices {
                            println!("{}: {} ({:.1} GB) - {:?}",
                                d.id, d.name,
                                d.size_bytes as f64 / 1_073_741_824.0,
                                d.device_type);
                        }
                    }
                }
                Err(e) => eprintln!("Error listing devices: {}", e),
            }
        }
    }

    Ok(())
}
