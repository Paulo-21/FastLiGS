use std::process::exit;
use std::time::Instant;
use clap::Parser;
use clap::arg;
use crate::parser;
use crate::nn::*;
pub enum Format {
    Apx,
    Cnf
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Problem {
    DS, DC, SE
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Semantics {
    CO,ST,SST,STG,ID,PR
}
#[derive(Debug, Clone)]
pub struct Task {
    pub problem : Problem,
    pub _problem_name : String,
    pub semantics : Semantics,
    pub argument : usize,
    pub verbose : bool,
}

#[derive(Parser, Debug)]
#[command(author="Paul Cibier", version, about="This tool can solve all the problems in the approximate track of ICCMA 2023",
long_about = None, arg_required_else_help = true)]
struct Cli {
    #[arg(short, long)]
    /// Quary argument for credulous and skeptical acceptance
    argument : Option<String>,
    #[arg(short='f', long="input_AF")]
    /// Path of the file containing the AF.
    input_af : Option<String>,
    #[arg(long="fo")]
    /// Format of the file containing the AF.
    input_format : Option<String>,
    #[arg(short = 'p', long="task")]
    /// A computational problem supported by the solver (e.g. DC-CO, DS-PR).
    task : Option<String>,
    #[arg(long)]
    /// Prints the supported computational problems and exits
    problems : bool,
    /// Print details of the execution time of each part of the solution
    /// "FILE ; GROUNDED ; HEURISTIC ; OUTPUT "
    #[arg(short, long, verbatim_doc_comment)]
    verbose : bool,
}

pub fn launcher() {
    let cli = Cli::parse();

    if cli.problems { // Print support problem if --problems
        print_supported_problems();
        exit(0);
    }
    
    let arg_name = cli.argument.clone();
    let argument_name = match arg_name {
        Some(arg) => { arg.parse::<usize>().unwrap()-1 },
        None => {
            eprintln!("Expected an argument with -a");
            exit(1);
        }
    };
    let pr_sm = cli.task.clone();
    let _problem_name = pr_sm.clone().unwrap();
    let (problem, semantics) = match pr_sm {
        Some(t) => {
            if !t.contains('-') {
                eprintln!("Error parsing command-line arguments\n");
                exit(1);
            }
            let mut r = t.split('-');
            let problem = String::from(r.next().unwrap());
            let problem = match problem.as_str() {
                "DC" => Problem::DC,
                "DS" => Problem::DS,
                _ => { eprintln!("This problem is not handled by the program at this time"); exit(1);}
            };
            let semantics = String::from(r.next().unwrap());
            let semantics = match semantics.as_str() {
                "ST" => Semantics::ST,
                "SST" => Semantics::SST,
                //"STG" => Semantics::Stg,
                "ID" => Semantics::ID,
                "PR" => Semantics::PR,
                "CO" => Semantics::CO,
                _ => { eprintln!("This semantics is not handled by the program at this time"); exit(1);}
            };
            (problem, semantics)
        },
        None => {
            eprintln!("expected a problem and a semantic");
            exit(1) 
        }
    };
    let task = Task { problem, _problem_name, semantics, argument : argument_name, 
        verbose : cli.verbose,
    };
    let file = cli.input_af.clone().unwrap();
    let file_path = file.as_str();
    let start = Instant::now();
    let af = if let Some(fo) = cli.input_format {
        if fo == "apx" {
            parser::get_input(file_path, Format::Apx)
        }
        else {
             parser::get_input(file_path, Format::Cnf)
        }
    } else {
        parser::get_input(file_path, Format::Cnf)
    };
    if task.verbose {
        print!("{};",start.elapsed().as_millis() as f32 / 1000.0);
    }

    af_nn(af, task);
}

fn print_supported_problems() {
    println!("[DC-CO,DC-ST,DC-SST,DC-ID,DS-PR,DS-ST,DS-SST]");
}