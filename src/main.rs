// Cargo.toml
// [dependencies]
// image = "0.24.9"
// clap = { version = "4.5.4", features = ["derive"] }

use std::fs::{self, File, create_dir_all};
use std::io::{self, BufReader, BufRead, Read, Write};
use std::path::Path;
use image::GrayImage;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Unpacks MNIST .ubyte files into PNG images and label files
    Unpack {
        #[arg(long, default_value = "./data")]
        input_dir: String,
        #[arg(long, default_value = "./output")]
        output_dir: String,
    },
    /// Packs PNG images and label files back into MNIST .ubyte files
    Pack {
        #[arg(long, default_value = "./output")]
        input_dir: String,
        #[arg(long, default_value = "./output")]
        output_dir: String,
    },
}


fn unpack_images(image_path: &Path, output_dir: &Path, prefix: &str) -> io::Result<()> {
    let file = File::open(image_path)?;
    let mut reader = BufReader::new(file);

    // Read header (big-endian)
    let mut header = [0u8; 16];
    reader.read_exact(&mut header)?;
    let magic = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let num_images = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
    let rows = u32::from_be_bytes([header[8], header[9], header[10], header[11]]);
    let cols = u32::from_be_bytes([header[12], header[13], header[14], header[15]]);

    assert_eq!(magic, 2051, "Not a valid MNIST image file: {:?}", image_path);
    println!("Images file: {:?}", image_path);
    println!("Images: {}, Size: {}x{}", num_images, rows, cols);

    create_dir_all(output_dir)?;

    let image_size = (rows * cols) as usize;
    let mut buffer = vec![0u8; image_size];

    for i in 0..num_images {
        reader.read_exact(&mut buffer)?;

        let img = GrayImage::from_raw(cols, rows, buffer.clone()).unwrap();
        let filename = output_dir.join(format!("{}_{:05}.png", prefix, i));
        img.save(&filename).unwrap();
    }
    println!("Saved {} images to {:?}", num_images, output_dir);
    Ok(())
}

fn unpack_labels(label_path: &Path, output_path: &Path) -> io::Result<()> {
    let file = File::open(label_path)?;
    let mut reader = BufReader::new(file);

    // Read header (big-endian)
    let mut header = [0u8; 8];
    reader.read_exact(&mut header)?;
    let magic = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let num_labels = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);

    assert_eq!(magic, 2049, "Not a valid MNIST label file: {:?}", label_path);
    println!("
Labels file: {:?}", label_path);
    println!("Labels: {}", num_labels);

    let mut labels = vec![0u8; num_labels as usize];
    reader.read_exact(&mut labels)?;

    let mut output_file = File::create(output_path)?;
    for label in labels {
        writeln!(output_file, "{}", label)?;
    }

    println!("Saved {} labels to {:?}", num_labels, output_path);
    Ok(())
}

fn pack_labels(input_path: &Path, output_path: &Path) -> io::Result<()> {
    println!("
Packing labels from: {:?}", input_path);
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let labels: Vec<u8> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u8>().unwrap())
        .collect();

    let num_labels = labels.len() as u32;

    let mut output_file = File::create(output_path)?;

    // Write header
    output_file.write_all(&2049u32.to_be_bytes())?; // Magic number
    output_file.write_all(&num_labels.to_be_bytes())?; // Number of labels

    // Write labels
    output_file.write_all(&labels)?;

    println!("Saved {} labels to {:?}", num_labels, output_path);
    Ok(())
}

fn pack_images(input_dir: &Path, output_path: &Path) -> io::Result<()> {
    println!("Packing images from: {:?}", input_dir);
    let mut paths: Vec<_> = fs::read_dir(input_dir)?
        .map(|r| r.unwrap().path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "png"))
        .collect();
    paths.sort();

    let num_images = paths.len() as u32;
    let (rows, cols) = (28u32, 28u32);

    let mut output_file = File::create(output_path)?;

    // Write header
    output_file.write_all(&2051u32.to_be_bytes())?; // Magic number
    output_file.write_all(&num_images.to_be_bytes())?;
    output_file.write_all(&rows.to_be_bytes())?;
    output_file.write_all(&cols.to_be_bytes())?;

    for path in paths {
        let img = image::open(path).unwrap().to_luma8();
        let pixels = img.into_raw();
        output_file.write_all(&pixels)?;
    }

    println!("Saved {} images to {:?}", num_images, output_path);
    Ok(())
}


fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Unpack { input_dir, output_dir } => {
            println!("--- Starting MNIST Unpack ---");
            let input_path = Path::new(input_dir);
            let output_path = Path::new(output_dir);

            create_dir_all(output_path.join("train"))?;
            create_dir_all(output_path.join("test"))?;

            // Unpack training data
            unpack_images(&input_path.join("train-images-idx3-ubyte"), &output_path.join("train"), "train")?;
            unpack_labels(&input_path.join("train-labels-idx1-ubyte"), &output_path.join("train-labels.txt"))?;

            // Unpack test data
            unpack_images(&input_path.join("t10k-images-idx3-ubyte"), &output_path.join("test"), "test")?;
            unpack_labels(&input_path.join("t10k-labels-idx1-ubyte"), &output_path.join("test-labels.txt"))?;
            println!("
--- Unpack Finished ---");
        }
        Commands::Pack { input_dir, output_dir } => {
            println!("--- Starting MNIST Pack ---");
            let input_path = Path::new(input_dir);
            let output_path = Path::new(output_dir);
            create_dir_all(output_path)?;

            // Pack training data
            pack_labels(&input_path.join("train-labels.txt"), &output_path.join("packed-train-labels-idx1-ubyte"))?;
            pack_images(&input_path.join("train"), &output_path.join("packed-train-images-idx3-ubyte"))?;

            // Pack test data
            pack_labels(&input_path.join("test-labels.txt"), &output_path.join("packed-test-labels-idx1-ubyte"))?;
            pack_images(&input_path.join("test"), &output_path.join("packed-test-images-idx3-ubyte"))?;
            println!("
--- Pack Finished ---");
        }
    }

    Ok(())
}
