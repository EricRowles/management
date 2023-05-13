

use rand::prelude::*;
// use rand::distributions::{Distribution, Uniform};

use crate::work::names;

pub struct Work {
    pub contracts: Vec<Contract>,
    pub unions: Vec<Union>,
    pub money: u32,
    pub parachute: u32,
    union_id: u32,
}

impl Work {
    pub fn new() -> Work {
        let mut work = Work {
            contracts: Vec::new(),
            unions: Vec::new(),
            money: 0,
            parachute: 0,
            union_id: 0,
        };
        for _ in 0..2 {
            work.add_contract(
                format!("{} {}", 
                    names::select_from_list(names::VERB),
                    names::select_from_list(names::ANIMALS)
                ).as_str()
            );
        }
        for _ in 0..5 {
            work.add_union();
        }
        work
    }

    pub fn roll_contracts(&mut self) {
        println!("Rolling contracts");
        for contract in &mut self.contracts {
            contract.roll_difficulty();
            println!("{}: {}", contract.name, contract.difficulty);
        }
    }

    pub fn assign_unions_to_contracts(&mut self) {
        println!("Assigning unions to contracts");
        let mut input = String::new();
        loop {
            self.print_unions();
            self.print_contracts();
            println!("Select union to move, or enter 'c' to cancel:");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input == "c\n" {
                break;
            }
            let index = input.trim().parse::<usize>().unwrap();
            if index >= self.unions.len() {
                println!("Invalid index");
                continue;
            }

            self.print_contracts();
            println!("Select contract to move union {} to, or enter 'c' to cancel:", index);
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input == "c\n" {
                continue;
            }
            let contract_id = input.trim().parse::<usize>().unwrap();
            if contract_id >= self.contracts.len() {
                println!("Invalid index");
                continue;
            }
            for contract in &mut self.contracts {
                contract.members.retain(|x| *x != self.unions[index]);
            }
            self.contracts[contract_id].members.push(self.unions[index].clone());
        }
    }

    pub fn roll_attempts(&mut self) {
        for contract in &mut self.contracts {
            if contract.roll_attempt() {
                println!("{}: Success!", contract.name);
                self.money += contract.difficulty
            } else {
                println!("{}: Failure!", contract.name);
            }
        }
    }

    pub fn spend_money(&mut self) {
        let mut input = String::new();
        while self.money > 0 {
            let mut options = Vec::new();
            for union in self.unions.iter() {
                if !union.paid {
                    options.push(
                        SelectionOption::new(
                            format!("Pay {}", union.workers_str()), 
                            (union.members.len() as u32)*2,
                            // union.pay_union,
                        )
                    );
                }
            }
            for union in self.unions.iter() {
                if (*union).seeking_promotion() {
                    options.push(
                        SelectionOption::new(
                            format!("Promote {}", union.workers_str()), 
                            union.members.len() as u32, 
                            // union.promote,
                        )
                    );
                }
            }
            options.push(
                SelectionOption::new(
                    "Buy worker".to_string(), 
                    2, 
                    // self.add_union,
                )
            );
            println!("You have {} money, with a parachute of {}. Enter 'c' to add all remaining money to your parachute, or select from the following:", self.money, self.parachute);
            for (index, option) in options.iter().enumerate() {
                println!("{}: {} ({})", index, option.name, option.cost);
            }
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input == "c\n" {
                break;
            }
            let selection = input.trim().parse::<u32>().unwrap();
            if selection >= options.len() as u32 {
                println!("Invalid index");
                continue;
            }
            options[selection as usize].select(&mut self.money);
        }
        self.parachute += self.money;
    }

    pub fn improve_contracts(&mut self) {
        let mut input = String::new();
        println!("Add a contract? (y/n)");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input == "y\n" {
            println!("Enter contract name:");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            self.add_contract(input.trim());
        }
    }

    pub fn add_contract(&mut self, name:&str) {
        let contract = Contract::new(name.to_string());
        self.contracts.push(contract)
    }

    pub fn add_union(&mut self) {
        self.unions.push(Union::new(self.union_id));
        self.union_id += 1
    }

    pub fn print_unions(&self) {
        for (index, union) in self.unions.iter().enumerate() {
            println!("{}: {}", index, union.workers_str());
        }
    }

    pub fn print_contracts(&self) {
        for (index, contract) in self.contracts.iter().enumerate() {
            println!("{}: {} - {}", index, contract.name, contract.difficulty);
            for (index, union) in self.unions.iter().enumerate() {
                if contract.members.contains(union) {
                    println!("  {}: {}", index, union.total_sum());
                }
            }
        }
    }
}

pub struct SelectionOption {
    pub name: String,
    pub cost: u32,
    // pub function: fn() -> (),
}
impl SelectionOption {
    pub fn new(name: String, cost: u32) -> SelectionOption {
        SelectionOption {
            name: name,
            cost: cost,
            // function: function,
        }
    }
    pub fn select(&self, money: &mut u32) {
        println!("{} selected (NYI)", self.name);
        // (self.function)();
        *money -= self.cost;
    }
}


pub struct Contract {
    pub name: String,
    pub members: Vec<Union>,
    pub difficulty: u32,
}

impl Contract {
    pub fn new(name: String) -> Contract {
        Contract {
            name: name,
            members: Vec::new(),
            difficulty: 0,
        }
    }

    pub fn roll_difficulty(&mut self) -> () {
        self.difficulty = thread_rng().gen_range(1..11)
    }

    pub fn roll_attempt(&mut self) -> bool {
        let mut effort = 0;
        for union in &mut self.members {
            for member in &mut union.members {
                member.roll();
            }
            effort += union.total_sum();
            union.paid = false
        }
        return effort >= self.difficulty
    }
}

#[derive(Clone)]
pub struct Union {
    pub id: u32,
    pub members: Vec<Worker>,
    pub paid: bool,
    pub newly_hired: bool,
    pub denied_promotion: bool,
}

impl Union {
    pub fn new(id: u32) -> Union {
        let mut union = Union {
            id: id,
            members: Vec::new(),
            paid: true,
            newly_hired: true,
            denied_promotion: false,
        };
        union.members.push(Worker::new());
        union
    }

    pub fn total_sum(&self) -> u32 {
        let mut sum = 0;
        for member in &self.members {
            sum += member.value;
        }
        if self.newly_hired {
            sum -= self.members.len() as u32;
        }
        if self.denied_promotion {
            sum -= self.members.len() as u32;
        }
        sum
    }

    pub fn seeking_promotion(&self) -> bool {
        let mut seeking_promotion = false;
        for member in self.members.iter() {
            if member.value == 6+member.ability {
                seeking_promotion = true;
            }
        }
        seeking_promotion
    }

    pub fn workers_str(&self) -> String {
        let mut workers_str = "".to_string();
        for worker in &self.members {
            workers_str += &worker.name;
            workers_str += ", ";
        }
        workers_str
    }

    // pub fn pay_union(&mut self) {
    //     self.paid = true;
    // }

    // pub fn deny_promotion(&mut self) {
    //     self.denied_promotion = true;
    // }

    // pub fn promote(&mut self) {
    //     for worker in &mut self.members {
    //         worker.promote()
    //     }
    // }

}

impl PartialEq for Union {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
pub struct Worker {
    pub name: String,
    pub ability: u32,
    pub value: u32,
}
impl Worker {
    pub fn new() -> Worker {
        Worker {
            name: names::select_from_list(names::FIRST_NAMES).to_string(),
            ability: 0,
            value: 0,
        }
    }

    // pub fn promote(&mut self) {
    //     println!("{} Promoted!", self.name);
    //     self.ability += 1;
    // }

    pub fn roll(&mut self) {
        self.value = thread_rng().gen_range(1..7 + self.ability) + self.ability;
        println!("{} Rolled!", self.value);
    }
}