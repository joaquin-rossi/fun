use std::io::Read;
use fun::{ast::Decl, terms::Term, typs, typs::Typ, vals::Val, ProgramContext};

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let mut src = String::new();
    match args.len() {
        0 => unreachable!(),
        2 => {
            if &args[1] == "-" {
                let mut stdin = std::io::stdin();
                stdin.read_to_string(&mut src)?;
            } else {
                src = std::fs::read_to_string(&args[1])?;
            }
        }
        _ => {
            eprintln!("usage: {} [file]", args[0]);
            std::process::exit(1);
        }
    };

    handle(&src);

    Ok(())
}

fn handle(src: &str) {
    let program = grammar::ProgramParser::new().parse(src).unwrap();

    let mut ctx = ProgramContext::default().insert_val(
        "print",
        &typs::func!(Typ::atom("Int"), Typ::atom("Unit")),
        &Val::native(|n| {
            println!("{}", n);
            Ok(Val::Unit)
        }),
    );

    let mut main = None;

    for decl in &program {
        match decl {
            Decl::Type(_, _) => {
                todo!();
            }
            Decl::Let(name, term) => {
                if let Some(_) = ctx.get_val(&name) {
                    eprintln!("Term defined twice at the global scope: {}", name);
                    return;
                }

                if let Some(_) = main {
                    eprintln!("Term defined after main: {}", name);
                    return;
                }

                if name == "main" {
                    main = Some(term);
                } else {
                    ctx = match ctx.insert_term(&name, &term) {
                        Ok(ctx) => ctx,
                        Err(why) => {
                            eprintln!("{:?}", why);
                            return;
                        }
                    }
                }
            }
        }
    }

    match main {
        None => eprintln!("No main function defined"),
        Some(main) => {
            match ctx.typ(&main) {
                Ok(typ) => {
                    if typ != Typ::func(Typ::atom("Unit"), Typ::atom("Unit")) {
                        eprintln!(
                            "Invalid type defined for main ({}): it must have type Unit -> Unit",
                            typ
                        );
                        return;
                    }
                }
                Err(why) => {
                    eprintln!("Type error: {}", why);
                    return;
                }
            }

            match ctx.eval(&Term::app(main.clone(), Term::var("Unit"))) {
                Err(why) => {
                    eprintln!("Eval error: {}", why);
                    return;
                }
                _ => {}
            }
        }
    }
}
