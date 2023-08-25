use aoc2018::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
struct Task {
    name: char,
    state: TaskState,
    duration: usize,
    deps: HashSet<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum TaskState {
    Ignored,
    Waiting,
    Running(usize),
    Done,
}

impl Task {
    fn new(name: char, duration: usize) -> Task {
        let state = TaskState::Ignored;
        let deps = HashSet::new();
        Task {
            name,
            state,
            duration,
            deps,
        }
    }

    fn is_waiting(&self) -> bool {
        match self.state {
            TaskState::Waiting => true,
            _ => false,
        }
    }

    fn is_running(&self) -> bool {
        match self.state {
            TaskState::Running(_) => true,
            _ => false,
        }
    }

    fn is_done(&self) -> bool {
        match self.state {
            TaskState::Done => true,
            _ => false,
        }
    }

    fn is_ignored(&self) -> bool {
        match self.state {
            TaskState::Ignored => true,
            _ => false,
        }
    }

    fn is_complete(&self) -> bool {
        self.is_done() || self.is_ignored()
    }

    fn ensure_waiting(&mut self) -> &mut Self {
        self.state = TaskState::Waiting;
        self
    }

    fn add_dep(&mut self, dep: usize) -> &mut Self {
        self.deps.insert(dep);
        self
    }

    fn rm_dep(&mut self, dep: usize) {
        self.deps.remove(&dep);
    }

    fn maybe_run(&mut self, solution: &mut String) -> bool {
        if self.is_waiting() && self.deps.is_empty() {
            self.state = TaskState::Running(self.duration);
            solution.push(self.name);
            true
            } else {
            false
        }
    }

    fn run_step(&mut self) -> bool {
        use TaskState::{Done, Running};
        if let Running(1) = self.state {
            self.state = Done;
            true
        } else if let Running(t) = &mut self.state {
            *t -= 1;
            false
        } else {
            false
        }
    }
}

fn tick(tasks: &mut [Task], solution: &mut String, workers: usize) -> bool {
    for i in 0..26 {
        if tasks[i].run_step() {
            tasks.iter_mut().for_each(|t| t.rm_dep(i))
        }
    }
    let running_tasks = tasks.iter().filter(|t| t.is_running()).count();
    let free_workers = workers - running_tasks;

    for _ in 0..free_workers {
        'inner: for task in tasks.iter_mut() {
            if task.maybe_run(solution) {
                break 'inner
            }
        }
    }

    tasks.iter().find(|t| !(t.is_complete())).is_some()
}

fn main() {
    let workers = 5;
    let task_base_time = 60;
    let task_extra_time = 1;
    // Initialize tasks.
    let mut tasks = (0..26)
        .map(|x| {
            Task::new(
                char::from_u32((x as usize + 'A' as usize) as u32).unwrap(),
                task_base_time + ((x + 1) * task_extra_time),
            )
        })
        .collect::<Vec<Task>>();

    // Parse input
    for task in input_lines::<String>() {
        let mut task = task.chars();
        let before = char_as_usize(task.nth(5).unwrap());
        let after = char_as_usize(task.nth(30).unwrap());

        tasks[before].ensure_waiting();
        tasks[after].ensure_waiting().add_dep(before);
    }

    // Start time
    let mut time = 0;
    let mut solution = String::from("");
    while tick(&mut tasks, &mut solution, workers) {
        time += 1;
    }
    println!(" - Done in {:?} seconds.", time);
    println!(" - Order: {}", solution);
}

fn char_as_usize(c: char) -> usize {
    c as usize - 'A' as usize
}
