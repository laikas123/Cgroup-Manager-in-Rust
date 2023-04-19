use regex::Regex;


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
















