use std::{error::Error, fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    // remember: Rust lets you return values from if statements
    let res = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for l in res {
        println!("{}", l);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /*
     * The rust book returns "Result<Config, &str>" and the error returned is "Err("not enought arguments")".
     *
     * The return type of the method has been changed to "Result<Config, String>" because I am creating a String
     * via "format!". Once I do that, I am creating an owned type => "new" now owns that String => if I were to return
     * the &str from that String, via ".as_str()" for example, I would be extracting a slice from the String => I would
     * be referencing the value, i.e. borrow.
     * This cannot be done because when the method exits, the String cease to exist, i.e. dangling reference!!!
     *
     * The Rust book can return "Err("not enought arguments")" because the string inside "Err" has been stored in the
     * program binary, i.e. it is a "&'static str", its lifetime is bound to the program binary.
     *
     */
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!(
                "not enought arguments: found {}, want 3",
                args.len()
            ));
        }
        // clone not efficient because it copies data, but easier than using lifetimes for now
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for l in content.lines() {
        if l.contains(query) {
            res.push(l);
        }
    }
    res
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for l in content.lines() {
        // l and query are not being modified as "to_lowercase" returns a String
        if l.to_lowercase().contains(&query.to_lowercase()) {
            res.push(l)
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "me";
        let content = "hey\nit's me\nnot mr. MEME";

        assert_eq!(vec!["it's me"], search_case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "ME";
        let content = "hey\nit's me\nnot mr. MEME";

        assert_eq!(
            vec!["it's me", "not mr. MEME"],
            search(query, content)
        );
    }
}
