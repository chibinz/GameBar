use std::collections::HashSet;
use crate::console::Console;

use std::io::prelude::*;
use std::process::exit;

pub struct Debugger<'a>
{
    console   : &'a mut Console,
    breakpoint: HashSet<u32>,
    command   : Vec<String>,
}

impl<'a> Debugger<'a>
{
    pub fn new(c: &'a mut Console) -> Self
    {
        Self
        {
            console   : c,
            breakpoint: HashSet::new(),
            command   : vec![String::from("s")],
        }
    }

    pub fn run(&mut self)
    {
        loop
        {   
            self.console.print();
            
            self.prompt();
            self.dispatch();
        }
    }

    pub fn prompt(&mut self)
    {
        print!("(debug) ");
        std::io::stdout().flush().ok().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if !input.trim().is_empty()
        {
            self.command.clear();

            for str in input.split_whitespace()
            {
                self.command.push(str.to_string());
            }
        }
    }

    pub fn dispatch(&mut self)
    {
        match self.command[0].as_str()
        {
            "s" => self.console.step(),
            // "p" => self.console.print(),
            "c" => self.continue_run(),
            "b" => self.insert_breakpoint(),
            "d" => self.delete_breakpoint(),
            "l" => self.list_breakpoint(),
            "q" => exit(0),
            _   => println!("Invalid input"),
        }
    }

    fn continue_run(&mut self)
    {
        while !self.breakpoint_hit()
        {
            self.console.step()
        }
    }

    fn insert_breakpoint(&mut self)
    {
        if self.command.len() < 2
        {
            println!("Please specify breakpoint")
        }
        else
        {
            match u32::from_str_radix(self.command[1].as_str(), 16)
            {
                Err(_) => println!("Invalid breakpoint"),
                Ok(e)  => {self.breakpoint.insert(e);},
            };
        }
    }

    fn delete_breakpoint(&mut self)
    {
        if self.command.len() < 2
        {
            println!("Please specify breakpoint")
        }
        else
        {
            match u32::from_str_radix(self.command[1].as_str(), 16)
            {
                Err(_) => println!("Invalid breakpoint"),
                Ok(e)  => {self.breakpoint.remove(&e);},
            }
        }
    }

    fn list_breakpoint(&self)
    {
        for b in self.breakpoint.iter()
        {
            println!("{:#8x}", b);
        }
    }

    fn breakpoint_hit(&mut self) -> bool
    {
        self.breakpoint.contains(&(self.console.cpu.r[15] - 2))
    }
}