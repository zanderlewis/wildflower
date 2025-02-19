use super::eval::eval_args;
use super::eval::BUILT_INS;
use super::primitive;
use super::FUNCTIONS;
use super::{Err, Exp, Number};
use crate::{could_not, expected};

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::f64::consts::PI;

#[derive(Clone)]
pub struct Env {
    pub data: BTreeMap<String, Exp>,
    pub outer: Option<Rc<RefCell<Env>>>,
}

pub fn default_env() -> Rc<RefCell<Env>> {
    let mut data: BTreeMap<String, Exp> = BTreeMap::new();

    data.insert("pi".to_string(), Exp::Num(Number::from(PI)));
    data.insert("=".to_string(), Exp::Primitive(primitive::lisp_eq));
    data.insert(">".to_string(), Exp::Primitive(primitive::lisp_gt));
    data.insert(">=".to_string(), Exp::Primitive(primitive::lisp_gte));
    data.insert("<".to_string(), Exp::Primitive(primitive::lisp_lt));
    data.insert("<=".to_string(), Exp::Primitive(primitive::lisp_lte));
    data.insert("*".to_string(), Exp::Primitive(primitive::lisp_mul));
    data.insert("+".to_string(), Exp::Primitive(primitive::lisp_add));
    data.insert("-".to_string(), Exp::Primitive(primitive::lisp_sub));
    data.insert("/".to_string(), Exp::Primitive(primitive::lisp_div));
    data.insert("^".to_string(), Exp::Primitive(primitive::lisp_exp));
    data.insert("<<".to_string(), Exp::Primitive(primitive::lisp_shl));
    data.insert(">>".to_string(), Exp::Primitive(primitive::lisp_shr));
    data.insert("rem".to_string(), Exp::Primitive(primitive::lisp_rem));
    data.insert("cos".to_string(), Exp::Primitive(primitive::lisp_cos));
    data.insert("acos".to_string(), Exp::Primitive(primitive::lisp_acos));
    data.insert("asin".to_string(), Exp::Primitive(primitive::lisp_asin));
    data.insert("atan".to_string(), Exp::Primitive(primitive::lisp_atan));
    data.insert("sin".to_string(), Exp::Primitive(primitive::lisp_sin));
    data.insert("tan".to_string(), Exp::Primitive(primitive::lisp_tan));
    data.insert("trunc".to_string(), Exp::Primitive(primitive::lisp_trunc));
    data.insert("shell".to_string(), Exp::Primitive(primitive::lisp_shell));
    data.insert("string".to_string(), Exp::Primitive(primitive::lisp_string));
    data.insert(
        "string->binary".to_string(),
        Exp::Primitive(primitive::lisp_string_binary),
    );
    data.insert(
        "binary->string".to_string(),
        Exp::Primitive(primitive::lisp_binary_string),
    );
    data.insert(
        "binary->number".to_string(),
        Exp::Primitive(primitive::lisp_binary_number),
    );
    data.insert(
        "number->binary".to_string(),
        Exp::Primitive(primitive::lisp_number_binary),
    );
    data.insert(
        "number->string".to_string(),
        Exp::Primitive(primitive::lisp_number_string),
    );
    data.insert(
        "string->number".to_string(),
        Exp::Primitive(primitive::lisp_string_number),
    );
    data.insert("type".to_string(), Exp::Primitive(primitive::lisp_type));
    data.insert("parse".to_string(), Exp::Primitive(primitive::lisp_parse));
    data.insert("list".to_string(), Exp::Primitive(primitive::lisp_list));
    data.insert("sort".to_string(), Exp::Primitive(primitive::lisp_sort));
    data.insert("unique".to_string(), Exp::Primitive(primitive::lisp_unique));
    data.insert(
        "contains?".to_string(),
        Exp::Primitive(primitive::lisp_contains),
    );
    data.insert("slice".to_string(), Exp::Primitive(primitive::lisp_slice));
    data.insert("chunks".to_string(), Exp::Primitive(primitive::lisp_chunks));
    data.insert("length".to_string(), Exp::Primitive(primitive::lisp_length));
    data.insert("concat".to_string(), Exp::Primitive(primitive::lisp_concat));
    data.insert(
        "number/type".to_string(),
        Exp::Primitive(primitive::lisp_number_type),
    );
    data.insert(
        "regex/find".to_string(),
        Exp::Primitive(primitive::lisp_regex_find),
    );
    data.insert(
        "string/split".to_string(),
        Exp::Primitive(primitive::lisp_string_split),
    );
    data.insert(
        "string/trim".to_string(),
        Exp::Primitive(primitive::lisp_string_trim),
    );
    data.insert(
        "file/size".to_string(),
        Exp::Primitive(primitive::lisp_file_size),
    );
    data.insert(
        "file/exists?".to_string(),
        Exp::Primitive(primitive::lisp_file_exists),
    );
    data.insert(
        "file/open".to_string(),
        Exp::Primitive(primitive::lisp_file_open),
    );
    data.insert(
        "file/read".to_string(),
        Exp::Primitive(primitive::lisp_file_read),
    );
    data.insert(
        "file/write".to_string(),
        Exp::Primitive(primitive::lisp_file_write),
    );
    data.insert(
        "file/close".to_string(),
        Exp::Primitive(primitive::lisp_file_close),
    );
    data.insert(
        "socket/connect".to_string(),
        Exp::Primitive(primitive::lisp_socket_connect),
    );
    data.insert(
        "socket/listen".to_string(),
        Exp::Primitive(primitive::lisp_socket_listen),
    );
    data.insert(
        "socket/accept".to_string(),
        Exp::Primitive(primitive::lisp_socket_accept),
    );
    data.insert("host".to_string(), Exp::Primitive(primitive::lisp_host));
    data.insert("dict".to_string(), Exp::Primitive(primitive::lisp_dict));
    data.insert("get".to_string(), Exp::Primitive(primitive::lisp_get));
    data.insert("put".to_string(), Exp::Primitive(primitive::lisp_put));
    data.insert("date".to_string(), Exp::Primitive(primitive::lisp_date));
    data.insert("vector".to_string(), Exp::Primitive(primitive::lisp_vector));
    data.insert("block".to_string(), Exp::Primitive(primitive::lisp_block));
    data.insert("keyword".to_string(), Exp::Primitive(primitive::lisp_keyword));
    data.insert("struct".to_string(), Exp::Primitive(primitive::lisp_struct));
    data.insert("enum".to_string(), Exp::Primitive(primitive::lisp_enum));

    data.insert("struct/get".to_string(), Exp::Primitive(primitive::lisp_struct_get));
    data.insert("keyword/name".to_string(), Exp::Primitive(primitive::lisp_keyword_name));
    data.insert("enum/info".to_string(), Exp::Primitive(primitive::lisp_enum_info));
    data.insert("vector/push".to_string(), Exp::Primitive(primitive::lisp_vector_push));
    data.insert("vector/pop".to_string(), Exp::Primitive(primitive::lisp_vector_pop));
    data.insert("list/append".to_string(), Exp::Primitive(primitive::lisp_list_append));
    data.insert("list/prepend".to_string(), Exp::Primitive(primitive::lisp_list_prepend));
    data.insert("dict/keys".to_string(), Exp::Primitive(primitive::lisp_dict_keys));
    data.insert("dict/values".to_string(), Exp::Primitive(primitive::lisp_dict_values));
    data.insert("block/eval".to_string(), Exp::Primitive(primitive::lisp_block_eval));
    data.insert("block/length".to_string(), Exp::Primitive(primitive::lisp_block_length));
    data.insert("block/to_list".to_string(), Exp::Primitive(primitive::lisp_block_to_list));
    data.insert("block/to_vector".to_string(), Exp::Primitive(primitive::lisp_block_to_vector));
    data.insert("block/length".to_string(), Exp::Primitive(primitive::lisp_block_length));


    // Setup autocompletion
    *FUNCTIONS.lock() = data
        .keys()
        .cloned()
        .chain(BUILT_INS.map(String::from))
        .collect();

    Rc::new(RefCell::new(Env { data, outer: None }))
}

pub fn env_keys(env: &Rc<RefCell<Env>>) -> Result<Vec<String>, Err> {
    let env = env.borrow_mut();
    let mut keys: Vec<String> = env.data.keys().cloned().collect();
    if let Some(outer_env) = &env.outer {
        keys.extend_from_slice(&env_keys(outer_env)?);
    }
    Ok(keys)
}

pub fn env_get(key: &str, env: &Rc<RefCell<Env>>) -> Result<Exp, Err> {
    let env = env.borrow_mut();
    match env.data.get(key) {
        Some(exp) => Ok(exp.clone()),
        None => match &env.outer {
            Some(outer_env) => env_get(key, outer_env),
            None => could_not!("find symbol '{}'", key),
        },
    }
}

pub fn env_set(key: &str, val: Exp, env: &Rc<RefCell<Env>>) -> Result<Exp, Err> {
    let mut env = env.borrow_mut();
    match env.data.get(key) {
        Some(_) => {
            env.data.insert(key.to_string(), val.clone());
            Ok(val)
        }
        None => match &env.outer {
            Some(outer_env) => env_set(key, val, outer_env),
            None => could_not!("find symbol '{}'", key),
        },
    }
}

enum InnerEnv {
    Function,
    Macro,
}

fn inner_env(
    kind: InnerEnv,
    params: &Exp,
    args: &[Exp],
    outer: &mut Rc<RefCell<Env>>,
) -> Result<Rc<RefCell<Env>>, Err> {
    let mut args = match kind {
        InnerEnv::Function => eval_args(args, outer)?,
        InnerEnv::Macro => args.to_vec(),
    };
    let mut data: BTreeMap<String, Exp> = BTreeMap::new();
    match params {
        Exp::Sym(s) => {
            data.insert(s.clone(), Exp::List(args));
        }
        Exp::List(list) => {
            let mut list = list.to_vec();
            let n = list.len();
            let m = args.len();

            let mut is_variadic = false;
            if n > 0 {
                if let Exp::List(l) = &list[n - 1] {
                    if l.len() == 2 && l[0] == Exp::Sym("splice".to_string()) {
                        if let Exp::Sym(_) = &l[1] {
                            is_variadic = true;
                            list[n - 1] = l[1].clone();
                            if n <= m {
                                let rest = args.drain((n - 1)..).collect();
                                args.push(Exp::List(rest));
                            }
                        }
                    }
                }
            }
            let m = args.len();

            if n != m {
                let s = if n != 1 { "s" } else { "" };
                let a = if is_variadic { "at least " } else { "" };
                return expected!("{}{} argument{}, got {}", a, n, s, m);
            }
            for (exp, arg) in list.iter().zip(args.iter()) {
                if let Exp::Sym(s) = exp {
                    data.insert(s.clone(), arg.clone());
                } else {
                    return expected!("params to be a list of symbols");
                }
            }
        }
        _ => return expected!("params to be a list"),
    }
    Ok(Rc::new(RefCell::new(Env {
        data,
        outer: Some(Rc::new(RefCell::new(outer.borrow_mut().clone()))),
    })))
}

pub fn function_env(
    params: &Exp,
    args: &[Exp],
    outer: &mut Rc<RefCell<Env>>,
) -> Result<Rc<RefCell<Env>>, Err> {
    inner_env(InnerEnv::Function, params, args, outer)
}

pub fn macro_env(
    params: &Exp,
    args: &[Exp],
    outer: &mut Rc<RefCell<Env>>,
) -> Result<Rc<RefCell<Env>>, Err> {
    inner_env(InnerEnv::Macro, params, args, outer)
}
