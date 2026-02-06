use chrono::{Datelike, Local, Timelike};
use hidapi::{HidApi, HidDevice};
use std::{thread, time::Duration};

const VENDOR_ID: u16 = 0x0c45;
const PRODUCT_ID: u16 = 0x8009;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;

    // Find Interface 3
    let device_info = api
        .device_list()
        .find(|d| {
            d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID && d.interface_number() == 3
        })
        .ok_or("Keyboard Interface 3 not found")?;

    let device = api.open_path(device_info.path())?;

    let now = Local::now();
    let (year, month, day) = (now.year() as u8 % 100, now.month() as u8, now.day() as u8);
    let (hour, minute, second) = (now.hour() as u8, now.minute() as u8, now.second() as u8);

    // Data: 00 (ID) + 04 18
    sync_step(&device, &[0x00, 0x04, 0x18], 1)?;

    // Data: 00 (ID) + 04 28 ... 01
    let mut p2 = [0u8; 65];
    p2[0] = 0;
    p2[1] = 0x04;
    p2[2] = 0x28;
    p2[9] = 0x01;
    sync_step(&device, &p2, 2)?;

    // Time Packet
    // Data: 00 (ID) + 00 01 5A + Time + AA 55
    let mut p3 = [0u8; 65];
    p3[0] = 0;
    p3[1] = 0x00;
    p3[2] = 0x01;
    p3[3] = 0x5a;
    p3[4] = year;
    p3[5] = month;
    p3[6] = day;
    p3[7] = hour;
    p3[8] = minute;
    p3[9] = second;
    p3[63] = 0xaa;
    p3[64] = 0x55;
    sync_step(&device, &p3, 3)?;

    // Data: 00 (ID) + 04 02
    sync_step(&device, &[0x00, 0x04, 0x02], 4)?;

    Ok(())
}

fn sync_step(device: &HidDevice, data: &[u8], _step: u8) -> Result<(), Box<dyn std::error::Error>> {
    device.send_feature_report(data)?;

    let mut buf = [0u8; 65];
    buf[0] = 0x00;
    let _ = device.get_feature_report(&mut buf);

    // STALL based on observed timings in Wireshark
    thread::sleep(Duration::from_millis(20));

    Ok(())
}
