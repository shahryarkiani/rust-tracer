use std::io;

mod bmp;

fn main() -> Result<(), io::Error>{
    let height: u32 = 64;
    let width: u32 = 64;
    assert!(width % 4 == 0);

    let image_size: usize = (height * width * 3) as usize;

    let mut image_data = vec![0u8; image_size];


    for i in 0..image_data.len() {
        image_data[i] = (i * (257 * i % 3)) as u8;
    }

    bmp::save_image("examples/test.bmp", width, height, image_data)?;
    
    Ok(())
}
