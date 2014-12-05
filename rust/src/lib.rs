use std::collections::btree_map::{
    BTreeMap,
    Occupied,
    Vacant,
};

#[deriving(PartialEq, PartialOrd, Eq, Show)]
pub struct Command {
    pub name: String,
    pub count: uint,
}

impl Command {
    pub fn weight(&self) -> uint {
        self.name.len() * self.count
    }
}

impl Ord for Command {
    fn cmp(&self, other: &Command) -> Ordering {
        other.weight().cmp(&self.weight())
    }
}

pub fn analyze_history(history: &str) -> Vec<Command> {
    let mut command_count: BTreeMap<&str,uint> = BTreeMap::new();

    for command in get_commands(history).iter() {
        match command_count.entry(*command) {
            Vacant(entry) => { entry.set(1); },
            Occupied(mut entry) => { *entry.get_mut() += 1 },
        }
    }

    commands_from_map(command_count)
}

fn commands_from_map(map: BTreeMap<&str,uint>) -> Vec<Command> {
    let mut commands: Vec<Command> = map
        .iter()
        .map(|entry| {
            let (name, count) = entry;
            Command { name: name.to_string(), count: *count }
        }).collect();

    commands.sort();

    commands
}

fn get_commands(history: &str) -> Vec<&str> {
    history
        .lines()
        .filter_map(|line| extract_command(line) )
        .collect()
}

fn extract_command(line: &str) -> Option<&str> {
    let mut words: Vec<&str>  = line.words().collect();
    words.reverse();
    words.pop();

    words.pop()
}

#[test]
fn analyze_history_works() {
    let history ="\
1  screen -S chi
2  git checkout -b pb-pry-rails
3  git commit -am 'Add pry-rails'
4  git push -u origin pb-pry-rails
5  git checkout -b pb-add-date-sent
6  nano vimrc
7  nano ~/.vimrc
8  pgrep vim
9  killall -9 vim
10  git grep date_received
11  git commit -m 'Add date_sent'
12  git push -u origin pb-add-date-sent
13  git checkout -b pb-coverage-fixups
14  rm -r app/validators/email_validator.rb
15  vim app/helpers/search_helper.rb
16  vim spec/models/risk_
17  vim spec/models/risk_trigger_spec.rb
18  vim spec/models/notice_search_result_spec.rb
19  be rake SPEC_OPTS=--no-drb
20  git push -u origin pb-coverage-fixups";

    let expected = vec![
        Command { name: "git".to_string(), count: 9 },
        Command { name: "vim".to_string(), count: 4 },
        Command { name: "nano".to_string(), count: 2 },
        Command { name: "killall".to_string(), count: 1 },
        Command { name: "screen".to_string(), count: 1 },
        Command { name: "pgrep".to_string(), count: 1 },
        Command { name: "rm".to_string(), count: 1 },
        Command { name: "be".to_string(), count: 1 },
    ];

    assert_eq!(analyze_history(history), expected);
}

#[test]
fn get_commands_works() {
    let history ="\
1  screen -S chi
2  git checkout -b pb-pry-rails";

    let expected = ["screen", "git"];

    assert_eq!(expected, get_commands(history));
}

#[test]
fn extract_command_works_for_command_line() {
    let line = "1 screen -S chi";

    assert_eq!(Some("screen"), extract_command(line));
}

#[test]
fn extract_command_works_for_garbage() {
    let line = "end";

    assert_eq!(None, extract_command(line));
}
