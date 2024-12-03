#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Args {
    file: String,
    rem: Vec<String>,
}

impl Args {
    pub fn get(&self, flag: &str, pos: usize) -> Option<String> {
        find_arg(&self.rem, flag)
            .0
            .or_else(|| self.rem.get(pos).cloned())
    }

    pub fn boolean_flag(&self, flag: &str) -> bool {
        let (arg, pos) = find_arg(&self.rem, flag);
        if let Some(arg) = arg {
            arg == "true"
        } else if let Some(pos) = pos {
            self.rem
                .get(pos + 1)
                .map(|it| it == "true" || it.starts_with("--"))
                .unwrap_or(true)
        } else {
            false
        }
    }

    pub fn file(&self) -> String {
        std::fs::read_to_string(&self.file).expect("a valid utf-8 file")
    }
}

fn parse_args<S: ToString>(args: Vec<S>) -> Args {
    let mut file = None;
    let mut args: Vec<_> = args.into_iter().map(|a| a.to_string()).collect();
    match args.len() {
        0 => (),
        1 => {
            file = Some(args.remove(0));
        }
        2 => match args[0].as_str() {
            "--source" => {
                file = Some(args[1].clone());
                args.remove(0);
                args.remove(0);
            }
            f => {
                file = Some(f.to_owned());
            }
        },
        _ => {
            let pos;
            (file, pos) = find_arg(&args, "--source");
            if let Some(pos) = pos {
                args.remove(pos);
                args.remove(pos);
            }
        }
    }

    Args {
        file: file.expect("a valid file path"),
        rem: args,
    }
}

pub fn args() -> Args {
    let mut args = std::env::args().collect::<Vec<_>>();
    args.remove(0);
    parse_args(args)
}

fn find_arg(args: &Vec<String>, flag: &str) -> (Option<String>, Option<usize>) {
    let pos = args
        .iter()
        .enumerate()
        .find(|(_, arg)| *arg == &flag)
        .map(|(i, _)| i);

    if let Some(pos) = pos {
        (args.get(pos + 1).cloned(), Some(pos))
    } else {
        (None, None)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    macro_rules! arg_test {
        (
            $test : ident -> $args: expr,
             file = $file: literal,
             rem = $rem: expr
         ) => {
            arg_test! {
                $test -> $args,
                file = $file,
                rem = $rem,
                then = |_| {}
            }
        };
        (
           $test : ident -> $args: expr,
            file = $file: literal,
            rem = $rem: expr,
            then = $then: expr
        ) => {
            #[test]
            fn $test() {
                let parsed = parse_args($args);
                assert_eq!(
                    parsed,
                    Args {
                        file: $file.into(),
                        rem: $rem.into_iter().map(String::from).collect()
                    }
                );
                ($then)(parsed)
            }
        };
    }

    arg_test! {
        args_with_file ->
        vec!["some-path"],
        file = "some-path",
        rem = Vec::<String>::new()
    }

    arg_test! {
        two_args_file_flag ->
        vec!["--source", "some-path"],
        file = "some-path",
        rem = Vec::<String>::new()
    }

    arg_test! {
        three_args_misc ->
        vec!["--source", "some-path", "other"],
        file = "some-path",
        rem = vec!["other"],
        then = |args: Args|{
            assert_eq!{
                args.get("--other", 0),
                Some("other".into())
            }
        }
    }

    arg_test! {
        three_args_misc_with_flags ->
        vec!["--source", "some-path", "--other", "other"],
        file = "some-path",
        rem = vec!["--other", "other"],
        then = |args: Args|{
            assert_eq!{
                args.get("--other", 0),
                Some("other".into())
            }
        }
    }

    arg_test! {
        args_with_bool_flag ->
        vec!["--source", "some-path", "--with-tolerance"],
        file = "some-path",
        rem = vec!["--with-tolerance"],
        then = |args: Args|{
            assert_eq!{
                args.boolean_flag("--with-tolerance"),
                true
            }
        }
    }

    arg_test! {
        args_with_bool_flag_false ->
        vec!["--source", "some-path"],
        file = "some-path",
        rem = Vec::<String>::new(),
        then = |args: Args|{
            assert_eq!{
                args.boolean_flag("--with-tolerance"),
                false
            }
        }
    }
}
