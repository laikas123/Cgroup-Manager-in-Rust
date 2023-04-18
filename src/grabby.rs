use regex::Regex;
//returns the first instance of a character and everything after it
//e.g. get_substring_to_end("W", "Hello World") ----> World
// 
//function args: c is substring to match, text is the text to match from
//
pub fn get_substring_to_end(c: &str, text: &str) -> Result<String, &'static str> {
    let pattern = format!(r"{}.*", &escape_special_chars(c));
    let re = Regex::new(&pattern).unwrap();
    let match_found = re.is_match(text);

    if !match_found {
        Err("No match found in text")
    }else{
        let caps = re.captures(text).unwrap();
        println!("{}", caps.len());
        let result = caps.get(0).map_or("", |m| m.as_str());
        Ok(result.to_string())
    }

}
#[test]
fn test_get_substring_to_end(){

    //should pass with World
    let mut c = "W";
    let mut text = "Hello World";

    match get_substring_to_end(c, text) {
        Ok(v) => assert_eq!(v, "World"),
        Err(e) => assert_eq!(0, 1),
    }
    
    //should fail, there is no T
    c = "T";
    text = "Hello World";

    match get_substring_to_end(c, text) {
        Ok(v) => assert_eq!(0, 1),
        Err(e) => assert_eq!(1, 1),
    }

    //should get \\\Users
    c = r"\\\";
    text = r"C:\\\Users";

    match get_substring_to_end(c, text) {
        Ok(v) => assert_eq!(v, r"\\\Users"),
        Err(e) => assert_eq!(0, 1),
    }

    //should get "Bobby"
    c = r#""Bo"#;
    text = r#"Ricky "Bobby""#;


    match get_substring_to_end(c, text) {
        Ok(v) => assert_eq!(v, r#""Bobby""#),
        Err(e) => assert_eq!(0, 1),
    }

    c = r"-";
    text = r"A-B";


    match get_substring_to_end(c, text) {
        Ok(v) => assert_eq!(v, r"-B"),
        Err(e) => assert_eq!(0, 1),
    }


    //should return {xyz}
    c = r"{xyz}";
    text = r"{abc}{xyz}{lmo}";


    match get_substring_to_end(c, text) {
        Ok(v) => assert_eq!(v, r"{xyz}{lmo}"),
        Err(e) => assert_eq!(0, 1),
    }

}

//
//Supposedly these characters: . ^ $ * + ? ( ) [ ] { } \ | need escaped for perl like regex
//and supposedly this regex crate is perl like
//I found additionally that & needs to for this regex implementation
pub fn escape_special_chars(text: &str) -> String {
    //it's actually important backslash is done first
    //because otherwise a ton of backslashes would
    //be getting replaced that shouldn't...
    let mut result = str::replace(text, r"\", r"\\");
    result = str::replace(&result, r".", r"\.");
    result = str::replace(&result, r"^", r"\^");
    result = str::replace(&result, r"$", r"\$");
    result = str::replace(&result, r"*", r"\*");
    result = str::replace(&result, r"+", r"\+");
    result = str::replace(&result, r"?", r"\?");
    result = str::replace(&result, r"(", r"\(");
    result = str::replace(&result, r")", r"\)");
    result = str::replace(&result, r"[", r"\[");
    result = str::replace(&result, r"]", r"\]");
    result = str::replace(&result, r"{", r"\{");
    result = str::replace(&result, r"}", r"\}");
    result = str::replace(&result, r"|", r"\|");
    result = str::replace(&result, r"&", r"\&");
    result
}

#[test]
fn test_escape_special_chars() {
    let text = r".^$*+?()[]{}\|";
    assert_eq!(escape_special_chars(text), r"\.\^\$\*\+\?\(\)\[\]\{\}\\\|");
}




//
// assumes unix paths so forward slashes e.g. /home/logan/test.txt
//
pub fn get_every_part_of_filepath(filepath: &str) -> Result<Vec<String>, &'static str> {

    let pattern = r"[^/]*[^/]";
    let re = Regex::new(&pattern).unwrap();
    let match_found = re.is_match(filepath);



    if !match_found {
        Err("No match found in text")
    }else{
        let mut result: Vec<String> = Vec::new();

        for cap in re.captures_iter(filepath) {
            result.push(cap[0].to_string());
        }
        Ok(result)
    }

}


#[test]
fn test_get_every_part_of_filepath(){


    //test a normal case
    let mut filepath = "/home/logan/test.txt";
    let mut result = get_every_part_of_filepath(filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan".to_string(), "test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }

    //test with a space in one of the entries
    let mut filepath = "/home/logan dude/test.txt";
    let mut result = get_every_part_of_filepath(filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan dude".to_string(), "test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }

    //test with some weird characters
    let mut filepath = r"/home/logan dude $$$ ***/ \\\test.txt";
    let mut result = get_every_part_of_filepath(filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan dude $$$ ***".to_string(), r" \\\test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }

}



//this basically gets groups of text separated by the separator of your choosing
//e.g. for filenpaths you have "/home/logan/test.txt" you would use delimiter "/"
//and text "/home/logan/aikas" and expect to get (home, logan, test.txt)
//
//or it could work for spaces e.g. 
//text = "home logan test.txt" and sep = " " should also give (home, logan, text.txt)
pub fn get_text_separated_by_substring(sep: &str, text: &str) -> Result<Vec<String>, &'static str> {
    let pattern = format!(r"[^{}]*[^{}]", &escape_special_chars(sep), &escape_special_chars(sep),);
    let re = Regex::new(&pattern).unwrap();
    let match_found = re.is_match(text);

    

    if !match_found {
        Err("No match found in text")
    }else{
      
        let mut result: Vec<String> = Vec::new();

        for cap in re.captures_iter(text) {
            let mut val = cap[0].to_string();
            if val.ends_with('\n') {
                val.pop();
            }
            result.push(val);
        }
        Ok(result)
    }
}


#[test]
fn test_get_text_separated_by_substring(){


    //test separator "/""
    let mut filepath = "/home/logan/test.txt";
    let mut result = get_text_separated_by_substring("/", filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan".to_string(), "test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }


    //test separator " " (a single space)
    let mut filepath = "home logan test.txt";
    let mut result = get_text_separated_by_substring(" ", filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan".to_string(), "test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }

    //test separator "  " (a double space)
    let mut filepath = "home  logan  test.txt";
    let mut result = get_text_separated_by_substring(" ", filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan".to_string(), "test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }

    //test weird seperator 
    let mut filepath = "home??**&&logan??**&&test.txt";
    let mut result = get_text_separated_by_substring("??**&&", filepath);

    match result{
        Ok(v) =>  assert_eq!(vec!["home".to_string(), "logan".to_string(), "test.txt".to_string()], v),
        Err(e) => assert_eq!(0, 1),
    }

}





pub fn get_until_substring(c: &str, text: &str) -> Result<String, &'static str>{

    let pattern = format!(r"([^{}]*)", &escape_special_chars(c));
    let re = Regex::new(&pattern).unwrap();
    let match_found = re.is_match(text);

    if !match_found {
        Err("No match found in text")
    }else{
        let caps = re.captures(text).unwrap();
        println!("{}", caps.len());
        let result = caps.get(0).map_or("", |m| m.as_str());
        Ok(result.to_string())
    }

}

#[test]
fn test_get_until_substring(){

    //should pass with World
    let mut c = "W";
    let mut text = "Hello World";

    match get_until_substring(c, text) {
        Ok(v) => assert_eq!(v, "Hello "),
        Err(e) => assert_eq!(0, 1),
    }


    //should pass with /User
    let mut c = r"\";
    let mut text = r"/User\backwards/blah";

    match get_until_substring(c, text) {
        Ok(v) => assert_eq!(v, r"/User"),
        Err(e) => assert_eq!(0, 1),
    }

}














