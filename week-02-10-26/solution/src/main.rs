// Khung bài tuần 02-10-26: thay todo! bằng code của bạn.
// Các type/chữ ký và case dưới đây là đề bài, không phải bài đã nộp.
// Không cần derive, clone, Vec, Result hay iterator adapters.
mod measurement;
mod thermostat;
use crate::measurement::*;
use crate::thermostat::*;

enum Alarm {
    Silent,
    Light,
    Beep(u8),
}

fn intensity(alarm: &Alarm) -> u8 {
    match alarm {
        Alarm::Silent => 0,
        Alarm::Light => 1,
        Alarm::Beep(n) => {
            if *n >= 10 {
                10
            } else {
                *n
            }
        }
    }
}

enum CalibrationCommand {
    Set(i32),
    Reset,
}

struct Calibration {
    offset: i32,
}

impl Calibration {
    fn new() -> Self {
        Calibration { offset: 0 }
    }

    fn offset(&self) -> i32 {
        self.offset
    }

    fn apply(&mut self, command: &CalibrationCommand) -> bool {
        match command {
            CalibrationCommand::Reset => {
                self.offset = 0;
                true
            }
            CalibrationCommand::Set(value) => {
                if *value >= -10 && *value <= 10 {
                    self.offset = *value;
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn main() {
    println!("Bài 1");
    assert_eq!(temperature(&Measurement::Celsius(25)), Some(25));
    assert_eq!(temperature(&Measurement::Celsius(0)), Some(0));
    assert_eq!(temperature(&Measurement::Celsius(-40)), Some(-40));
    assert_eq!(temperature(&Measurement::Celsius(125)), Some(125));
    assert_eq!(temperature(&Measurement::Celsius(-41)), None);
    assert_eq!(temperature(&Measurement::Celsius(126)), None);
    assert_eq!(temperature(&Measurement::Celsius(i32::MIN)), None);
    assert_eq!(temperature(&Measurement::Celsius(i32::MAX)), None);
    assert_eq!(temperature(&Measurement::Offline), None);
    assert_eq!(temperature(&Measurement::Invalid(String::new())), None);
    let invalid = Measurement::Invalid(String::from("Cảm biến bị lỗi"));
    assert_eq!(temperature(&invalid), None);
    assert_eq!(temperature(&invalid), None);
    // Dữ liệu vẫn dùng được, thông báo không bị sửa.
    match &invalid {
        Measurement::Invalid(message) => assert_eq!(message, "Cảm biến bị lỗi"),
        _ => panic!("Hàm đọc đã thay đổi measurement"),
    }

    println!("Bài 2");
    assert_eq!(intensity(&Alarm::Silent), 0);
    assert_eq!(intensity(&Alarm::Light), 1);
    assert_eq!(intensity(&Alarm::Beep(0)), 0);
    assert_eq!(intensity(&Alarm::Beep(7)), 7);
    assert_eq!(intensity(&Alarm::Beep(10)), 10);
    assert_eq!(intensity(&Alarm::Beep(11)), 10);
    let loud = Alarm::Beep(u8::MAX);
    assert_eq!(intensity(&loud), 10);
    assert_eq!(intensity(&loud), 10);

    println!("Bài 3");
    let empty: [Measurement; 0] = [];
    assert_eq!(last_valid(&empty), None);
    let none = [
        Measurement::Offline,
        Measurement::Invalid(String::from("lỗi")),
        Measurement::Celsius(126),
        Measurement::Celsius(-41),
    ];
    assert_eq!(last_valid(&none), None);
    let mixed = [
        Measurement::Celsius(18),
        Measurement::Offline,
        Measurement::Celsius(23),
        Measurement::Invalid(String::from("lỗi")),
    ];
    assert_eq!(last_valid(&mixed), Some(23));
    assert_eq!(last_valid(&mixed), Some(23));
    assert_eq!(last_valid(&mixed[0..2]), Some(18));
    assert_eq!(last_valid(&[Measurement::Celsius(0)]), Some(0));
    assert_eq!(last_valid(&[Measurement::Celsius(-40)]), Some(-40));
    assert_eq!(
        last_valid(&[Measurement::Celsius(125), Measurement::Celsius(i32::MAX),]),
        Some(125)
    );

    println!("Bài 4");
    let mut calibration = Calibration::new();
    assert_eq!(calibration.offset(), 0);
    let command = CalibrationCommand::Set(7);
    assert!(calibration.apply(&command));
    assert!(calibration.apply(&command));
    assert_eq!(calibration.offset(), 7);
    assert!(!calibration.apply(&CalibrationCommand::Set(11)));
    assert_eq!(calibration.offset(), 7);
    assert!(!calibration.apply(&CalibrationCommand::Set(-11)));
    assert_eq!(calibration.offset(), 7);
    assert!(!calibration.apply(&CalibrationCommand::Set(i32::MIN)));
    assert_eq!(calibration.offset(), 7);
    assert!(!calibration.apply(&CalibrationCommand::Set(i32::MAX)));
    assert_eq!(calibration.offset(), 7);
    assert!(calibration.apply(&CalibrationCommand::Set(-10)));
    assert_eq!(calibration.offset(), -10);
    assert!(calibration.apply(&CalibrationCommand::Set(10)));
    assert_eq!(calibration.offset(), 10);
    assert!(calibration.apply(&CalibrationCommand::Set(0)));
    assert_eq!(calibration.offset(), 0);
    assert!(calibration.apply(&command));
    assert!(calibration.apply(&CalibrationCommand::Reset));
    assert_eq!(calibration.offset(), 0);
    assert!(calibration.apply(&CalibrationCommand::Reset));
    assert_eq!(calibration.offset(), 0);

    println!("Mini project");
    let mut room = Thermostat::new(String::from("Phòng làm việc"));
    assert_eq!(room.name(), "Phòng làm việc");
    assert_eq!(room.target(), 22);
    assert_eq!(room.last(), None);
    match room.mode() {
        Mode::Off => {}
        _ => panic!("Constructor phải đặt mode Off"),
    }
    assert!(!room.is_heating());
    let empty_name = Thermostat::new(String::new());
    assert_eq!(empty_name.name(), "");
    assert_eq!(empty_name.target(), 22);
    assert_eq!(empty_name.last(), None);
    assert!(!empty_name.is_heating());

    // Lệnh lỗi trước khi có mẫu: không tự tạo mẫu hay đổi mode.
    assert!(!room.apply(Command::Sample(Measurement::Offline)));
    assert!(!room.apply(Command::Sample(Measurement::Invalid(String::new()))));
    assert!(!room.apply(Command::SetTarget(i32::MIN)));
    assert_eq!(room.name(), "Phòng làm việc");
    assert_eq!(room.target(), 22);
    assert_eq!(room.last(), None);
    match room.mode() {
        Mode::Off => {}
        _ => panic!("Lệnh lỗi đã đổi mode"),
    }

    assert!(room.apply(Command::SetMode(Mode::Auto)));
    assert_eq!(room.last(), None);
    assert!(!room.is_heating());
    assert!(room.apply(Command::Sample(Measurement::Celsius(18))));
    assert!(room.is_heating());
    assert_eq!(room.last(), Some(18));

    // Mỗi lần thất bại đều kiểm tra toàn bộ trạng thái.
    let rejected = [
        Command::SetTarget(15),
        Command::SetTarget(31),
        Command::SetTarget(i32::MAX),
        Command::Sample(Measurement::Offline),
        Command::Sample(Measurement::Invalid(String::from("Mất tín hiệu"))),
        Command::Sample(Measurement::Celsius(-41)),
        Command::Sample(Measurement::Celsius(126)),
        Command::Sample(Measurement::Celsius(i32::MIN)),
        Command::Sample(Measurement::Celsius(i32::MAX)),
        Command::Rename(String::new()),
        Command::Rename(String::from(" \t\n ")),
    ];
    // Vòng for sở hữu từng Command; đây là for trên array, không dùng Vec.
    for command in rejected {
        assert!(!room.apply(command));
        assert_eq!(room.name(), "Phòng làm việc");
        assert_eq!(room.target(), 22);
        assert_eq!(room.last(), Some(18));
        match room.mode() {
            Mode::Auto => {}
            _ => panic!("Lệnh lỗi đã đổi mode Auto"),
        }
        assert!(room.is_heating());
    }

    assert!(room.apply(Command::Sample(Measurement::Celsius(22))));
    assert!(!room.is_heating());
    assert!(room.apply(Command::Sample(Measurement::Celsius(23))));
    assert!(!room.is_heating());
    assert!(room.apply(Command::SetTarget(30)));
    assert_eq!(room.target(), 30);
    assert_eq!(room.last(), Some(23));
    assert!(room.is_heating());
    assert!(room.apply(Command::SetTarget(16)));
    assert_eq!(room.target(), 16);
    assert_eq!(room.last(), Some(23));
    assert!(!room.is_heating());
    assert!(room.apply(Command::Sample(Measurement::Celsius(-40))));
    assert_eq!(room.last(), Some(-40));
    assert!(room.is_heating());
    assert!(room.apply(Command::Sample(Measurement::Celsius(125))));
    assert_eq!(room.last(), Some(125));
    assert!(!room.is_heating());

    assert!(room.apply(Command::SetMode(Mode::Manual(true))));
    assert_eq!(room.target(), 16);
    assert_eq!(room.last(), Some(125));
    assert!(room.is_heating());
    assert!(room.apply(Command::Rename(String::from("  Góc học Rust  "))));
    assert_eq!(room.name(), "  Góc học Rust  ");
    assert_eq!(room.target(), 16);
    assert_eq!(room.last(), Some(125));
    match room.mode() {
        Mode::Manual(true) => {}
        _ => panic!("Rename không được đổi mode"),
    }
    assert!(room.is_heating());

    // Thất bại trong Manual vẫn giữ nguyên mode và dữ liệu.
    assert!(!room.apply(Command::Rename(String::from("\n\t"))));
    assert!(!room.apply(Command::SetTarget(31)));
    assert!(!room.apply(Command::Sample(Measurement::Offline)));
    assert_eq!(room.name(), "  Góc học Rust  ");
    assert_eq!(room.target(), 16);
    assert_eq!(room.last(), Some(125));
    match room.mode() {
        Mode::Manual(true) => {}
        _ => panic!("Lệnh lỗi đã đổi mode Manual"),
    }
    assert!(room.is_heating());

    assert!(room.apply(Command::SetMode(Mode::Manual(false))));
    assert!(!room.is_heating());
    assert!(room.apply(Command::Sample(Measurement::Celsius(-40))));
    assert!(!room.is_heating());
    match room.mode() {
        Mode::Manual(false) => {}
        _ => panic!("Sample không được đổi mode"),
    }
    assert!(room.apply(Command::SetMode(Mode::Off)));
    assert!(!room.is_heating());
    assert_eq!(room.target(), 16);
    assert_eq!(room.last(), Some(-40));
    assert!(room.apply(Command::SetMode(Mode::Auto)));
    assert!(room.is_heating());
    assert_eq!(room.name(), "  Góc học Rust  ");

    let mut manual = Thermostat::new(String::from("Chưa có mẫu"));
    assert!(manual.apply(Command::SetMode(Mode::Manual(true))));
    assert_eq!(manual.last(), None);
    assert!(manual.is_heating());
    assert!(manual.apply(Command::SetMode(Mode::Manual(false))));
    assert!(!manual.is_heating());

    println!("Tất cả case đã pass — có thể nộp bài để review.");
}
