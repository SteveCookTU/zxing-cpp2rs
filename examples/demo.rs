use zxing_cpp2rs::{ImageView, read_barcode};
static IMG: &[u8] = include_bytes!("img.png");
fn main() -> anyhow::Result<()> {
    let image = ImageView::new(IMG)?;
    let result = read_barcode(image);

    println!("Format: {}", result.format());
    println!("Text: {}", result.text());

    Ok(())
}