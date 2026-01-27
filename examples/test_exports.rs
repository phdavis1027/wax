use wax::Filter;

fn main() {
    let _ = wax::message();
    let _ = wax::message::param();
    let _ = wax::iq();
    let _ = wax::iq::param();
    let _ = wax::presence();
    let _ = wax::presence::param();
    let _ = wax::id("test");
    let _ = wax::id::param();
}
