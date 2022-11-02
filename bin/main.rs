extern crate termion;

use d_stuff::*;

fn main() {
    // let h = '─';
    // let v = '│';

    // let tl = '┌';
    // let tr = '┐';
    // let bl = '└';
    // let br = '┘';

    // let t = '┬';
    // let b = '┴';
    // let l = '├';
    // let r = '┤';
    // let c = '┼';

    // let R = '╣';
    // let V = '║';
    // let TR = '╗';
    // let BR = '╝';
    // let BL = '╚';
    // let TL = '╔';
    // let B = '╩';
    // let T = '╦';
    // let L = '╠';
    // let H = '═';
    // let C = '╬';

    // let c = "-⏳-⏱-⌚-⚠-❗-❌-✅-👍-👎-🔍-🔎-💡-⭐-▶-►-≠-≤-≥-¬-∨-∧-⇒-❓-✓✔✗✘";

    let mut pretty = Pretty::new();

    let item = Entry::new(
        Status::Success,
        Text::new(
            "Parser",
            termion::style::Bold.to_string(),
            termion::color::Fg(termion::color::Blue).to_string(),
        ),
        Some(Text::new(
            "COMPLETED",
            termion::style::Reset.to_string(),
            termion::color::Fg(termion::color::Green).to_string(),
        )),
        vec![],
    );
    pretty.add(item);

    let message = Message::new(
        None,
        Text::new(
            r#"let b: Bool
let i: Int = j + 1 // single line comment
let r: Real
/* multi 
    lines
    comment
*/
let bb: Bool = not b 
let j: Int
let rr: Real = i / 10
       
let k: -10..100 = i
let f(i: Int, b: Bool, r: Real, j: 1..10): Bool = false
       
constraint cst1 = 
    r > 2.5 and 
    not(j <= 5)
constraint cst2 = b => j > 0"#,
            termion::style::Reset.to_string(),
            termion::color::White.fg_str(),
        ),
    );
    let item = Entry::new(
        Status::Info,
        Text::new(
            "Model",
            termion::style::Bold.to_string(),
            termion::color::Fg(termion::color::Blue).to_string(),
        ),
        None,
        vec![message],
    );
    pretty.add(item);

    let item = Entry::new(
        Status::Failure,
        Text::new(
            "Parser",
            termion::style::Bold.to_string(),
            termion::color::Fg(termion::color::Blue).to_string(),
        ),
        Some(Text::new(
            "ERROR",
            termion::style::Reset.to_string(),
            termion::color::Fg(termion::color::Red).to_string(),
        )),
        vec![
            Message::new(
                Some(Text::new(
                    "unreconized token",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                Text::new(
                    "'('",
                    termion::style::Bold.to_string(),
                    termion::color::White.fg_str(),
                ),
            ),
            Message::new(
                Some(Text::new(
                    "file",
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                )),
                Text::new(
                    "files/example.sl:13:10",
                    termion::style::Reset.to_string(),
                    termion::color::LightBlue.fg_str(),
                ),
            ),
            Message::new(
                Some(Text::new(
                    "expecting",
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                )),
                Text::new(
                    "truc much machin bidule chose",
                    termion::style::Reset.to_string(),
                    termion::color::Reset.fg_str(),
                ),
            ),
        ],
    );
    pretty.add(item);
    pretty.print();
}
