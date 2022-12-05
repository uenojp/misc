#[derive(Debug)]
pub struct Command {
    n: usize,
    // 0-based
    from: usize,
    // 0-based
    to: usize,
}

impl Command {
    pub fn new(s: &str) -> Self {
        let command = s.split_whitespace().collect::<Vec<_>>();
        Self {
            n: command[1].parse::<usize>().unwrap(),
            from: command[3].parse::<usize>().unwrap() - 1,
            to: command[5].parse::<usize>().unwrap() - 1,
        }
    }
}

#[derive(Debug)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    pub fn new(s: &str) -> Self {
        let mut stacks = s.split('\n').collect::<Vec<_>>();
        // remove stack numbers
        stacks.pop();

        // map to Vec<(stack id, create)>
        let stack_info = stacks
            .into_iter()
            .flat_map(|stack| {
                stack
                    .chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| !c.is_whitespace())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let nstacks = stack_info.iter().map(|(i, _)| i).max().unwrap() + 1;
        let mut stacks = vec![vec![]; nstacks];
        for (stack_id, c) in stack_info {
            stacks[stack_id].insert(0, c);
        }

        Self(stacks)
    }

    pub fn apply(&mut self, command: Command) {
        let mut crates = vec![];
        for _ in 0..command.n {
            let c = self.0[command.from].pop().unwrap();
            crates.push(c);
        }
        for c in crates.into_iter().rev() {
            self.0[command.to].push(c);
        }
    }

    pub fn top_string(&self) -> String {
        self.0
            .iter()
            .map(|s| s.iter().last().unwrap())
            .collect::<String>()
    }
}

fn main() {
    let (stacks, procedure) = include_str!("../input").split_once("\n\n").unwrap();

    let mut stacks = Stacks::new(stacks);
    let procedure = procedure.split('\n').map(Command::new).collect::<Vec<_>>();
    for command in procedure {
        stacks.apply(command);
    }

    println!("{}", stacks.top_string());
}
