use fun::ast::Decl;
use fun::terms::Term;
use fun::typs;
use fun::typs::Typ;
use fun::vals::Val;
use fun::ProgramContext;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

fn main() {
    let src = r#"
let id = fun x:(Int -> Int -> Int) => x;

let main = fun _:Unit => {
    let x = id add;
    let y = x 1;
    let z = y 2;
    print z
};
"#;

    handle(src);
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
            Decl::Typ(_, _) => {
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
                        eprintln!("Invalid type defined for main ({}): it must have type Unit -> Unit", typ);
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
