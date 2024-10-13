use std::{cmp::Ordering, path::PathBuf};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use rten::Model;
use blow_up_monkey_calc::{calc, op, rec, win};
use winapi::um::winuser::SW_SHOW;

fn main() {
    let det_model_path = PathBuf::from_str("src/models/text-detection.rten").unwrap();
    let rec_model_path = PathBuf::from_str("src/models/text-recognition.rten").unwrap();

    let det_model = Model::load_file(det_model_path).expect("cannot load detection model");
    let rec_model = Model::load_file(rec_model_path).expect("cannot load recognition model");

    let engine = rec::create_engine(det_model, rec_model).expect("cannot create OCR Engine");
    println!("OCR Engine is ready");

    let hwnd = win::find_window(None, "M2101K9C").expect("cannot find phone controller");
    win::show_window(hwnd, SW_SHOW);
    if !win::set_foreground_window(hwnd) {
        panic!("cannot make phone controller foreground");
    }
    println!("Phone controller handled");

    let mut operator = op::Operator::new(0, 0, 0, 0, 1f64);
    while !win::is_iconic(hwnd) {
        let img = win::screenshot_on_window(hwnd).unwrap()
            .crop_imm(20, 200, 350, 150);
        // img.save("./test.png");
        let text = rec::recognize_only_text(&engine, img).expect("cannot recognize text");
        println!("text recognized: {}", text);

        let rect = win::get_window_rect(hwnd).expect("cannot get rect of controller");
        operator.area.x = (rect.left + 20) as u16;
        operator.area.y = (rect.top + 500) as u16;
        operator.area.width = (rect.right - rect.left - 20) as u16;
        operator.area.height = (rect.bottom - rect.top - 40 - operator.area.y as i32) as u16;

        if let Ok(comparation) = calc::prepare_comparation_input(text) {
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

        sleep(Duration::from_millis(500));
    }
}
