# inverse-kinematics

This is a Rust adaptation of [Coding Challenge: Inverse Kinematics](https://youtu.be/hbgDqyy8bIw)
using [eframe](https://crates.io/crates/eframe).

About application in a nutshell: You can generate "snake" with different parameters and its "head"
will follow your cursor.

To play with this, make sure you have [egui dependencies](https://github.com/maxwase/egui#demo)
installed, then run `cargo run --release`.

To try it on web, run `./start_web`. Run `setup_web.sh` if you're compiling this for the first time.
Directory name `docs/` was chosen for the GitHub pages support.

You also can try it online at https://maxwase.github.io/inverse-kinematics/

![](https://user-images.githubusercontent.com/23321756/147956344-b4fc7905-f2ee-4e1e-8c99-76c2a7bba2a0.gif)
