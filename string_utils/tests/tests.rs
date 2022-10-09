use cucumber::{given, when, then, World};
use test_assignment_02::string_utils;

#[derive(Debug, Default, World)]
pub struct TestWorld {
    text: String,
    reverse_text: String
}

#[given(expr = "a string of text, {string}")]
fn a_string(world: &mut TestWorld, text: String) {
    world.text = text;
}

#[when("The text is reversed")]
fn input_string(world: &mut TestWorld) {
    world.reverse_text = string_utils::reverse_string(&world.text)
}

#[then(expr = "the output is equal to, {string}")]
fn string_is_reversed(world: &mut TestWorld, reversed: String) {
    if !world.reverse_text.eq(&reversed) {
        panic!("strings are not equal!");
    }
}

fn main() {
    futures::executor::block_on(TestWorld::run("tests/features"));
}