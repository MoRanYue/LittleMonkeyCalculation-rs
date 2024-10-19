use std::{cmp::Ordering, path::PathBuf};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::env;
use rten::Model;
use blow_up_monkey_calc::{calc, op, rec, win};
use winapi::um::winuser::SW_SHOW;

fn main() {
    let mut args = env::args();

    let controller_title = args.next().expect("first arg must be the title of SCRCPY");
    let area_inp = args.next().expect("second arg must be reconition rect ('<left top X>,<left top Y>,<width>,<height>', points relate to left top corner of SCRCPY), example: 200,50,300,100");

    let rec_area_data: Vec<u16> = area_inp.split(",").map(|s| s.parse::<u16>().expect("area must be numbers")).collect();
    if rec_area_data.len() < 4 {
        panic!("area need 4 numbers, they are '<left top X>,<left top Y>,<width>,<height>'");
    }
    let rec_area = op::Rect {
        x: rec_area_data[0],
        y: rec_area_data[1],
        width: rec_area_data[2],
        height: rec_area_data[3]
    };

    let det_model_path = PathBuf::from_str("models/text-detection.rten").unwrap();
    let rec_model_path = PathBuf::from_str("models/text-recognition.rten").unwrap();

    let det_model = Model::load_file(det_model_path).expect("cannot load detection model");
    let rec_model = Model::load_file(rec_model_path).expect("cannot load recognition model");

    let engine = rec::create_engine(det_model, rec_model).expect("cannot create OCR Engine");
    println!("OCR Engine is ready");

    let hwnd = win::find_window(None, &controller_title).expect("cannot find phone controller");
    win::show_window(hwnd, SW_SHOW);
    if !win::set_foreground_window(hwnd) {
        panic!("cannot make phone controller foreground");
    }
    println!("Phone controller handled");

    let mut operator = op::Operator::new(0, 0, 0, 0, 1f64);
    while !win::is_iconic(hwnd) {
        win::show_window(hwnd, SW_SHOW);
        if !win::set_foreground_window(hwnd) {
            panic!("cannot make phone controller foreground");
        }

        let img = win::screenshot_on_window(hwnd).unwrap()
            .crop_imm(rec_area.x.into(), rec_area.y.into(), rec_area.width.into(), rec_area.height.into());
        // img.save("./test.png");
        let text = rec::recognize_only_text(&engine, img).expect("cannot recognize text");
        println!("text recognized: {}", text);

        let rect = win::get_window_rect(hwnd).expect("cannot get rect of controller");
        let w = rect.right - rect.left;
        let h = rect.bottom - rect.top;
        operator.area.width = (w as f64 * 0.9) as u16;
        operator.area.height = (h as f64 * 0.4) as u16;
        operator.area.x = (w as f64 * 0.05) as u16;
        operator.area.y = (h as f64 * 0.6) as u16;

        if let Ok(comparation) = calc::prepare_comparation_input(text) {
            operator.move_to_starting_point();
            println!("comparation: {} {} {}", comparation.0, match comparation.compare() {
                Ordering::Less => "<",
                Ordering::Greater => ">",
                _ => "?"
            }, comparation.1);
            match comparation.compare() {
                Ordering::Less => operator.draw_less_than(),
                Ordering::Greater => operator.draw_greater_than(),
                _ => ()
            }
            operator.move_to_starting_point();
        }
        else {
            println!("none");
        }

        sleep(Duration::from_millis(550));
    }
}
