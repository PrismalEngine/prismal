use prismal::prelude::futures::block_on;
use sandbox_app::run;

fn main() {
    block_on(run());
}
