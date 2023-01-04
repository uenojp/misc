use std::io;

fn tp(content: &str) -> String {
    let mut table: Vec<String> = Vec::new();

    for (line_number, line) in content.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if let Some(s) = table.get_mut(i) {
                if s.len() < line_number {
                    s.extend(std::iter::repeat(' ').take(line_number - s.len()));
                }
                s.push(c);
            } else {
                table.resize_with(i + 1, Default::default);
                table[i] = " ".repeat(line_number);
                table[i].push(c);
            }
        }
    }

    table.join("\n")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::read_to_string(io::stdin())?;
    println!("{}", tp(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = "abc
xy
i";
        let expected = "axi
by
c";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_2() {
        let input = "a
xy
ijk";
        let expected = "axi
 yj
  k";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_3() {
        let input = "abcdefgh
xy
ijklmnopqr";
        let expected = "axi
byj
c k
d l
e m
f n
g o
h p
  q
  r";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_4() {
        let input = "abcdefgh
xy
ijklmn";
        let expected = "axi
byj
c k
d l
e m
f n
g
h";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_including_empty_line() {
        let input = "abc

ijk";
        let expected = "a i
b j
c k";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_one_char() {
        let input = "a";
        let expected = "a";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_empyt() {
        let input = "";
        let expected = "";
        assert_eq!(expected, tp(input));
    }

    #[test]
    fn test_newline() {
        let input = "\n";
        let expected = "";
        assert_eq!(expected, tp(input));
    }
}
