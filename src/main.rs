mod map;
use map::arraymap;
fn main() {
    let mut map = arraymap::create_map();
    loop {
        arraymap::print_map(&mut map);
        println!("| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let direction = input.trim();
        if direction == "e" {
            println!("Ending the program.");
            break;
        }
        arraymap::move_player(&mut map, direction);
    }


}
