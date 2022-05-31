//program to confirm if string matches the regex pattern
use std::fmt;

struct Regex {
    p: String,
    t: String,
}
impl fmt::Debug for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Regex")
         .field("PATTERN", &self.p)
         .field("TYPE", &self.t)
         .finish()
    }
}

struct Solution {}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        //set regex_instructions to get instruction vector
        let regex_instructions: Vec<Regex> = get_instruction_vector(p);
        
        //get first regex instruction
        //iterate over characters in string
        let mut j = 0;
        let mut regex_instruction = &regex_instructions[j];
        let mut str: String = regex_instruction.p.clone();
        
        for (i, c) in s.chars().enumerate() {
            if c == str.chars().nth(0).unwrap() || str.chars().nth(0).unwrap()=='.' { str.remove(0); } 
            println!("i:{} c :{}", i,c);
          
            //if string is empty and regex instruction is a loop, increment j
            if str.is_empty() {
                if j == regex_instructions.len() - 1 && i == s.len() - 1 { return true; }
             

                //if regex instruction is a LOOP, and next character in s is same as c then str = regex instruction.p
                if regex_instruction.t == "LOOP" && (s.chars().nth(1).unwrap() == c || regex_instruction.p.chars().nth(0).unwrap() == '.') {
                    str = regex_instruction.p.clone();
                } else {
                    j += 1;
                    if j >= regex_instructions.len() {return false;}
                    regex_instruction = &regex_instructions[j];
                    str = regex_instruction.p.clone();
                }
            } else if regex_instruction.t=="LOOP"{
                j += 1;
                if j >= regex_instructions.len() {return false;}
                regex_instruction = &regex_instructions[j];
                str = regex_instruction.p.clone();

            }
   
        }
  
        //if regex instructions are not exhausted return false
        if j+1 < regex_instructions.len() {
            return false;
        }
        
        return true;
        
    }
}
fn get_instruction_vector(p: String) -> Vec<Regex> {
    let mut regex_instructions: Vec<Regex> = Vec::new();
    let mut temp_string: String = String::new();
    let mut temp_buffer: char = ' ';
    for c in p.chars() {
        if c == '*' {
            if !temp_string.is_empty() {
                let r = Regex {
                    p: temp_string.clone(),
                    t: String::from("SINGLE"),
                };
                regex_instructions.push(r);
            }
            
            let r = Regex {
                p: String::from(temp_buffer.to_string()),
                t: String::from("LOOP"),
            };
            regex_instructions.push(r);
            temp_buffer = ' ';
            temp_string = String::new();
        } else {
            if temp_buffer != ' ' {temp_string.push(temp_buffer)};
            temp_buffer = c;
        }
    }
    if temp_buffer != ' ' {temp_string.push(temp_buffer)};
    if !temp_string.is_empty() {
        let r = Regex {
            p: temp_string.clone(),
            t: String::from("SINGLE"),
        };
        regex_instructions.push(r);
    }
    println!("{:?}", regex_instructions);
    return regex_instructions;
}

fn main () {
    let s = "mississippi".to_string();
    let p = "mis*is*ip*.".to_string();
    let r = Solution::is_match(s, p);
    println!("{}", r);
    // Solution::is_match(s, p);

}