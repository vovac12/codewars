// https://www.codewars.com/kata/5868a68ba44cfc763e00008d/train/rust
// Esolang Interpreters #3 - Custom Paintf**k Interpreter

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Command {
    Unknown,
    MoveUp,
    MoveDown,
    MoveRight,
    MoveLeft,
    Flip,
    JumpForward,
    JumpBack,
}

impl From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            'n' => Command::MoveUp,
            'e' => Command::MoveRight,
            's' => Command::MoveDown,
            'w' => Command::MoveLeft,
            '*' => Command::Flip,
            '[' => Command::JumpForward,
            ']' => Command::JumpBack,
            _ => Command::Unknown
        }
    }
}

struct Programm {
    code: Vec<Command>,
    iterations: usize,
    width: usize,
    height: usize
}

impl Programm {
    fn new(code: &str, iterations: usize, width: usize, height: usize) -> Programm {
        let res = Programm{
            code: code.chars().filter_map(|x| match x.into() {
                Command::Unknown => None,
                x => Some(x)
            }).collect(),
            iterations,
            width,
            height
        };
        res
    }
    
    fn back_bracket(&self, idx: usize) -> Option<usize> {
        let mut depth: i32 = 0;
        let range = (0..=idx).rev();
        for i in range {
            match self.code.get(i)? {
                &Command::JumpBack => depth = depth.wrapping_add(1),
                &Command::JumpForward => {
                    depth = depth.wrapping_sub(1);
                    if depth == 0 {
                        return Some(i);
                    }
                },
                _ => ()
            }
        }
        None
    }
    
    fn forward_bracket(&self, idx: usize) -> Option<usize> {
        let mut depth: i32 = 0;
        let range = idx..self.code.len();
        for i in range {
            match self.code.get(i)? {
                &Command::JumpForward => depth = depth.wrapping_add(1),
                &Command::JumpBack => {
                    depth = depth.wrapping_sub(1);
                    if depth == 0 {
                        return Some(i);
                    }
                },
                _ => ()
            }
        }
        None
    }
    
    fn run(&self) -> Option<Vec<Vec<bool>>> {
        let mut data = vec![];
        data.resize_with(self.height, move || {
            let mut v = vec![];
            v.resize(self.width, false);
            v
        });
        let mut p = (0, 0);
        let mut idx = 0;
        for _ in 0..self.iterations {
            if idx >= self.code.len() {break;}
            match self.code.get(idx)? {
                Command::Flip => {
                    let val = data.get_mut(p.0)?.get_mut(p.1)?;
                    *val = !*val;
                },
                Command::MoveUp => p.0 = p.0.wrapping_add(self.height).wrapping_sub(1) % self.height,
                Command::MoveDown => p.0 = p.0.wrapping_add(1) % self.height,
                Command::MoveLeft => p.1 = p.1.wrapping_add(self.width).wrapping_sub(1) % self.width,
                Command::MoveRight => p.1 = p.1.wrapping_add(1) % self.width,
                Command::JumpForward if *data.get(p.0)?.get(p.1)? == false => {
                    idx = self.forward_bracket(idx)?;
                },
                Command::JumpBack if *data.get(p.0)?.get(p.1)? == true => {
                    idx = self.back_bracket(idx)?;
                },
                _ => ()
            }
            idx = idx.checked_add(1)?;
        }
        Some(data)
    }
}

fn interpreter(code: &str, iterations: usize,
    width: usize, height: usize) -> String {
    let prog = Programm::new(code, iterations, width, height);
    let res = match prog.run() {
        Some(x) => x,
        _ => return "".to_string()
    };
    res.iter()
       .map(|x| x
            .iter()
            .map(|&x| if x {'1'} else {'0'})
            .collect::<String>()
       ).collect::<Vec<String>>().join("\r\n")
}


fn main() {
    println!("{}", &interpreter("*[es*]", 9, 5, 6));
}


#[cfg(test)]
mod tests {
    use super::*;
    fn display_actual(s: &str) -> &str {
        println!("Actual\n{}", s);
        s
    }

    fn display_expected(s: &str) -> &str {
        println!("Expected\n{}", s);
        s
    }

    #[test]
    fn simple_cases() {
      assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9)), display_expected("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
      assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)), display_expected("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
      assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9)), display_expected("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
      assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
      assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
      assert_eq!(display_actual(&interpreter("*[es*]", 9, 5, 6)), display_expected("10000\r\n01000\r\n00100\r\n00000\r\n00000\r\n00000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
    }
}