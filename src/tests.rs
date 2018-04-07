
use foundation::GameState;
use utils::ipoint::IPoint;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn do_move() {
    let mut state = GameState::new(IPoint{x: 60, y: 60});
    println!("{}", state.get_view());
    let (_i1, lvl1) = state.game.levels.iter().next().unwrap();
    println!("{}", lvl1.build_layout().to_string());
    state.process_key("w");
    let (_i2, lvl2) = state.game.levels.iter().next().unwrap();
    println!("{}", lvl2.build_layout().to_string());
    state.process_key("s");
    let (_i3, lvl3) = state.game.levels.iter().next().unwrap();
    println!("{}", lvl3.build_layout().to_string());
}
