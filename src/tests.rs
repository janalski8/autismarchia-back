
use foundation::GameState;
use utils::ipoint::IPoint;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn do_move() {
    let mut state = GameState::new(IPoint{x: 60, y: 60});
    state.get_view();
    state.process_key("a");
    state.get_view();
    state.process_key("w");
    state.get_view();
    state.process_key("s");
    state.get_view();
}
