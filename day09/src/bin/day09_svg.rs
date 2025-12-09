use day09::INPUT;

fn main() {
    println!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="100000" height="100000" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
<polyline points=""#
    );
    print!("{}", INPUT);
    println!(
        r#"" stroke="red" stroke-width="400" fill="green" />
</svg>
"#
    );
}
