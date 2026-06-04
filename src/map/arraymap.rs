
pub fn print_map (map: &mut [[String; 5]; 5]) {
    for row in map.iter() {
        for (i, cell) in row.iter().enumerate() {
            if i == row.len() - 1 {
                print!("{}\n", cell);
            }else {
                 print!("{} ", cell);
            }
           
        }
    }

}
pub fn create_map() -> [[String; 5]; 5] {
    let map = [
        [String::from("*"), String::from("*"), String::from("*"), String::from("*"), String::from("*")],
        [String::from("*"), String::from("*"), String::from("*"), String::from("*"), String::from("*")],
        [String::from("*"), String::from("*"), String::from("x"), String::from("*"), String::from("*")],
        [String::from("*"), String::from("*"), String::from("*"), String::from("*"), String::from("*")],
        [String::from("*"), String::from("*"), String::from("*"), String::from("*"), String::from("*")]
    ];
    map
}
pub fn move_player(map: &mut [[String; 5]; 5], direction: &str) {
    let mut player_x = 0;
    let mut player_y = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == "x" {
                player_x = x;
                player_y = y;
            }
        }
    }
    
    map[player_y][player_x] = String::from("*");
    match direction {
        "w" => {
            if player_y > 0 {
                player_y -= 1;
            }
            else {
                println!("Can't move out of the map")
            }
        },
        "s" => {
            if player_y < map.len() - 1 {
                player_y += 1;
            }
            else {
                println!("Can't move out of the map")
            }
        },
        "a" => {
            if player_x > 0 {
                player_x -= 1;
            }
            else {
                println!("Can't move out of the map")
            }   
        },
        "d" => {
            if player_x < map[0].len() - 1 {
                player_x += 1;
            }else {
                println!("Can't move out of the map")
            }
        },
        _ => println!("Invalid direction! Use 'w', 's', 'a' or 'd   '.")
    }
    map[player_y][player_x] = String::from("x");
}