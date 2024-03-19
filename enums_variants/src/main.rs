
// use std::any::type_name_of_val;

// Define an enum representing different shapes
enum Shape {
    Circle(f64), // Radius of a circle
    Square(f64), // Length of one side for a square
}

enum FileSize1 {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

enum DiskType{
   SSD,
   HDD,
}

#[derive(Debug)]

enum DiskSize{
    KB(u32),
    MB(u32),
    GB(u32),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            Self::Bytes(bytes) => format!("{} bytes", bytes),
            Self::Kilobytes(kilobytes) => format!("{} KB", kilobytes),
            Self::Megabytes(megabytes) => format!("{} MB", megabytes),
            Self::Gigabytes(gigabytes) => format!("{} GB", gigabytes),
        }
    }
}

#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

// Options
fn divide(x:i32, y:i32)-> Option<i32>{
    if y == 0{
        None
    }
    else{
        Some(x/y)
    }
}


enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
    }
}


fn main() {
    // format_size
    println!("Enums and Variants: ");

    // let disk_type = DiskType::SSD;

    // println!("Type: {}", type_name_of_val(&disk_type));

    // match disk_type {
    //     DiskType::SSD => println!("Disk is SSD!"),
    //     DiskType::HDD => println!("Disk is HDD!"),
    // }

    // let disk_size = DiskSize::GB(128);

    // println!("{:?}", );

    // let wine1 = Wine {
    //     name: String::from("Chateau Margaux"),
    //     region: WineRegions::Bordeaux,
    // };

    // let wine2 = Wine {
    //     name: String::from("Barolo"),
    //     region: WineRegions::Tuscany,
    // };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    // supported_regions(wine1.region);
    // supported_regions(WineRegions::Rioja);

    //----------------------------------------------------------
    // let a = 10;
    // let b = 0;
    // let result = divide(a,b);

    // match result {
    //     Some(x) => println!("The result is : {}", x),
    //     None => println!("Error: Divided by zero!"),
    // }

    //-------------------------------------------------------


    let result = format_size(6888837399);
    println!("{}", result)

}
