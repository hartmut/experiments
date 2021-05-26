pub mod fileoperations;

use fileoperations::*;

fn insertcomma(i: &str) -> String {
    let mut a = i.to_string();
    a.replace_range(6..7, ",");
    a.replace_range(24..25, ",");
    a.replace_range(30..31, ",");
    a.replace_range(41..42, ",");
    a.replace_range(52..53, ",");
    a.replace_range(72..73, ",");
    a.replace_range(82..83, ",");
    a.replace_range(94..95, ",");
    a.replace_range(100..101, ",");
    a.replace_range(106..107 , ",");
    a.push_str("\n");
    a
}


fn main() {
    let inputfile = read_file_to_string("../../../Downloads/ELEMENTS.NUMBR".to_string());
    let mut lines = inputfile.lines();
    let mut output = "".to_string();

    let a = insertcomma(lines.next().unwrap());
    output.push_str(&a);
    lines.next();

    for s in lines {
        let a = insertcomma(s);
        output.push_str(&a);
    }

    write_string_to_file("bodiesorbit.csv".to_string(), &output);

}
