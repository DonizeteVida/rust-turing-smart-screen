use anyhow::{Context, Result};
use image::{EncodableLayout, ImageReader};
use turingsmartscreen::Display;

const DISPLAY_WIDTH: u16 = 320;
const DISPLAY_HEIGHT: u16 = 480;

fn rgb888_to_rgb565(buffer: &[u8; 3]) -> [u8; 2] {
    let r = buffer[0] as u16;
    let g = buffer[1] as u16;
    let b = buffer[2] as u16;

    //it will convert
    //RGB888 - 24 bits to
    //RGB565 - 16 bits
    let word = ((r & 0b11111000) << 8) | ((g & 0b11111100) << 3) | (b >> 3);
    [word as u8, (word >> 8) as u8]
}

fn display_draw_image(display: &mut Display, path: &str) -> Result<()> {
    display.send_draw_rect(0, 0, display.width - 1, display.height - 1)?;

    let bytes = ImageReader::open(path)?
        .decode()?
        .into_rgb8()
        .as_bytes()
        .as_chunks::<3>()
        .0
        .iter()
        .flat_map(rgb888_to_rgb565)
        .collect::<Vec<_>>();

    assert!(bytes.len() == (display.width as usize * display.height as usize * 2));

    display.send(bytes.as_ref())
}

fn main() -> Result<()> {
    let available_ports = Display::available_ports()?;
    println!("Available ports: {:#?}", available_ports);

    let usb_35_inchip_port = available_ports
        .into_iter()
        .filter(|port_info| port_info.serial_number.as_deref() == Some("USB35INCHIPSV2"))
        .collect::<Vec<_>>()
        .pop()
        .context("USB35INCHIPSV2 not found")?;

    let mut display = Display::new(usb_35_inchip_port, DISPLAY_WIDTH, DISPLAY_HEIGHT)?;
    println!("{:#?}", display);

    display_draw_image(&mut display, "docs/sample.jpg")?;

    Ok(())
}
