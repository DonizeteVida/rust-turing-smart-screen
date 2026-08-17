use anyhow::{Context, Result};
use serialport::{SerialPort, SerialPortType};

enum DisplayCommand {
    Clear = 102,
    ScreenOff = 108,
    ScreenOn = 109,
    DisplayBitmap = 197,
}

#[derive(Debug)]
pub struct Display {
    pub width: u16,
    pub height: u16,
    conn: Box<dyn SerialPort>,
}

#[derive(Debug)]
pub struct PortInfo {
    pub port_name: String,
    pub serial_number: Option<String>,
}

impl Display {
    pub fn available_ports() -> Result<Vec<PortInfo>> {
        let available_ports = serialport::available_ports()?;

        let turing_devices = available_ports
            .into_iter()
            .map(|info| PortInfo {
                port_name: info.port_name.to_owned(),
                serial_number: if let SerialPortType::UsbPort(ref info) = info.port_type {
                    info.serial_number.clone()
                } else {
                    None
                },
            })
            .collect::<Vec<_>>();

        Ok(turing_devices)
    }

    pub fn new(port_info: PortInfo, width: u16, height: u16) -> Result<Self> {
        let conn = serialport::new(port_info.port_name, 115_200).open()?;

        Ok(Self {
            width,
            height,
            conn,
        })
    }

    pub fn send(&mut self, bytes: &[u8]) -> Result<()> {
        if cfg!(debug_assertions) && false {
            println!("{:?}", bytes.to_ascii_lowercase());
        }

        Ok(self.conn.write_all(bytes)?)
    }

    fn send_statefull_command(
        &mut self,
        display_command: DisplayCommand,
        x: u16,
        y: u16,
        _x: u16,
        _y: u16,
    ) -> Result<()> {
        let mut buffer = [0u8; 6];

        //  X 10 bits, 8 MSB written, 2 remaining
        buffer[0] = (x >> 2) as u8;

        //  X 10 bits, 2 LSB written, 0 remaining
        //  Y 10 bits, 6 MSB written, 4 remaining
        buffer[1] = (x << 6) as u8 + (y >> 4) as u8;

        //  Y 10 bits, 4 LSB written, 0 remaining
        // _X 10 bits, 4 MSB written, 6 remaining
        buffer[2] = (y << 4) as u8 + (_x >> 6) as u8;

        // _X 10 bits, 6 LSB written, 0 remaining
        // _Y 10 bits, 2 MSB written, 8 remaining
        buffer[3] = (_x << 2) as u8 + (_y >> 8) as u8;

        // _Y 10 bits, 8 LSB written, 0 remaining
        buffer[4] = _y as u8;

        buffer[5] = display_command as u8;

        self.send(&buffer)
    }

    fn send_stateless_command(&mut self, display_command: DisplayCommand) -> Result<()> {
        let mut buf = [0u8; 6];
        buf[5] = display_command as u8;
        self.send(&buf)
    }

    fn clear(&mut self) -> Result<()> {
        self.send_stateless_command(DisplayCommand::Clear)
    }

    fn turn_on(&mut self) -> Result<()> {
        self.send_stateless_command(DisplayCommand::ScreenOn)
    }

    fn turn_off(&mut self) -> Result<()> {
        self.send_stateless_command(DisplayCommand::ScreenOff)
    }

    pub fn send_draw_rect(
        &mut self,
        start_x: u16,
        start_y: u16,
        end_x: u16,
        end_y: u16,
    ) -> Result<()> {
        self.send_statefull_command(
            DisplayCommand::DisplayBitmap,
            start_x,
            start_y,
            end_x,
            end_y,
        )
    }
}
