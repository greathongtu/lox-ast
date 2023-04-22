use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {

    define_ast(
        output_dir,
        &"Expr".to_string(),
        &vec![
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Literal> value".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
        ],
    )?;
    Ok(())
}

fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    write!(file, "{}", "use crate::token::*;\n")?;
    write!(file, "{}", "use crate::error::*;\n")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name); // Binary + Expr
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string() ,
            class_name,
            fields,
        });
    }

    write!(file, "\npub enum {base_name} {{\n");
    for t in &tree_types {
        write!(file, "    {}({}),\n", t.base_class_name, t.class_name);
    }
    write!(file, "}}\n\n");

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for f in &t.fields {
            write!(file, "    pub {},\n", f)?;
        }
        write!(file, "}}\n\n")?;
    }

    write!(file, "pub trait ExprVisitor<T> {{\n")?;
    for t in &tree_types {
        write!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    write!(file, "}}\n\n")?;
    // impl Expr {
    //     pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
    //         match self {
    //             Expr::Binary(expr) => visitor.visit_binary_expr(expr),
    //             Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
    //             Expr::Literal(expr) => visitor.visit_literal_expr(expr),
    //             Expr::Unary(expr) => visitor.visit_unary_expr(expr),
    //         }
    //     }
    // }
    write!(file, "impl {base_name} {{\n", base_name = base_name)?;
    write!(file, "    pub fn accept<T>(&self, visitor: &impl ExprVisitor<T>) -> Result<T, LoxError> {{\n")?;
    write!(file, "        match self {{\n")?;
    for t in &tree_types {
        write!(
            file,
            "            {}::{}(expr) => visitor.visit_{}_{}(expr),\n",
            base_name,
            t.base_class_name,
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
    }
    write!(file, "        }}\n")?;
    write!(file, "    }}\n")?;
    write!(file, "}}\n\n")?;

    Ok(())
}
