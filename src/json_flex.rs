use std::collections::HashMap;
use std::str::FromStr;
use std::ops::Index;
use std::boxed::Box;

const DEBUG: bool = false;

#[derive(RustcDecodable, Debug, Clone)]
pub enum JFObject {
    String(String),
    Integer(i64),
    Float(f64),
    Dictionary(HashMap<String, JFObject>),
    Array(Vec<JFObject>),
    Null,
    False,
    True,
}

impl JFObject {
    pub fn into_string(&self) -> Option<&String> {
        match self {
            &JFObject::String(ref v) => Some(v),
            _ => None,
        }
    }
    pub fn into_i64(&self) -> Option<&i64> {
        match self {
            &JFObject::Integer(ref v) => Some(v),
            _ => None,
        }
    }
    pub fn into_f64(&self) -> Option<&f64> {
        match self {
            &JFObject::Float(ref v) => Some(v),
            _ => None,
        }
    }
    pub fn into_hashmap(&self) -> Option<&HashMap<String, JFObject>> {
        match self {
            &JFObject::Dictionary(ref v) => Some(v),
            _ => None,
        }
    }
    pub fn into_vec(&self) -> Option<&Vec<JFObject>> {
        match self {
            &JFObject::Array(ref v) => Some(v),
            _ => None,
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            &JFObject::Null => true,
            _ => false,
        }
    }
    pub fn is_true(&self) -> bool {
        match self {
            &JFObject::True => true,
            _ => false,
        }
    }
    pub fn is_false(&self) -> bool {
        match self {
            &JFObject::False => true,
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            &JFObject::Array(_) => true,
            _ => false,
        }
    }
    pub fn is_dictionary(&self) -> bool {
        match self {
            &JFObject::Dictionary(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            &JFObject::String(_) => true,
            _ => false,
        }
    }
    pub fn is_integer(&self) -> bool {
        match self {
            &JFObject::Integer(_) => true,
            _ => false,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            &JFObject::Float(_) => true,
            _ => false,
        }
    }


    pub fn unwrap_string(&self) -> &String {
        match self {
            &JFObject::String(ref v) => v,
            _ => panic!(),
        }
    }
    pub fn unwrap_i64(&self) -> &i64 {
        match self {
            &JFObject::Integer(ref v) => v,
            _ => panic!(),
        }
    }
    pub fn unwrap_f64(&self) -> &f64 {
        match self {
            &JFObject::Float(ref v) => v,
            _ => panic!(),
        }
    }
    pub fn unwrap_hashmap(&self) -> &HashMap<String, JFObject> {
        match self {
            &JFObject::Dictionary(ref v) => v,
            _ => panic!(),
        }
    }
    pub fn unwrap_vec(&self) -> &Vec<JFObject> {
        match self {
            &JFObject::Array(ref v) => v,
            _ => panic!(),
        }
    }

    pub fn to_json(&self) -> String {
        match self {
            &JFObject::String(ref v) => format!("\"{}\"", v),
            &JFObject::Integer(ref v) => v.to_string(),
            &JFObject::Float(ref v) => v.to_string(),
            &JFObject::Dictionary(ref v) => {
                let mut string: String = "".to_owned();
                let mut is_first = true;
                for (k, v) in v {
                    if is_first {
                        is_first = false;
                    } else {
                        string.push(',');
                    }
                    string.push_str(&format!("\"{}\":", k));
                    string.push_str(&v.to_json());
                }
                format!("{{{}}}", string)
            }
            &JFObject::Array(ref v) => {
                let mut string: String = "".to_owned();
                let mut is_first = true;
                for i in v {
                    if is_first {
                        is_first = false;
                    } else {
                        string.push(',');
                    }
                    string.push_str(&i.to_json());
                }
                format!("[{}]", string)
            }
            &JFObject::Null => "null".to_owned(),
            &JFObject::False => "false".to_owned(),
            &JFObject::True => "true".to_owned(),
        }
    }
}

pub trait Unwrap<T> {
    fn unwrap(self) -> T;
}

impl Unwrap<String> for JFObject {
    fn unwrap(self) -> String {
        match self {
            JFObject::String(s) => s,
            _ => panic!(),
        }
    }
}
impl Unwrap<i64> for JFObject {
    fn unwrap(self) -> i64 {
        match self {
            JFObject::Integer(i) => i,
            _ => panic!(),
        }
    }
}
impl Unwrap<f64> for JFObject {
    fn unwrap(self) -> f64 {
        match self {
            JFObject::Float(i) => i,
            _ => panic!(),
        }
    }
}
impl Unwrap<HashMap<String, JFObject>> for JFObject {
    fn unwrap(self) -> HashMap<String, JFObject> {
        match self {
            JFObject::Dictionary(d) => d,
            _ => panic!(),
        }
    }
}
impl Unwrap<Vec<JFObject>> for JFObject {
    fn unwrap(self) -> Vec<JFObject> {
        match self {
            JFObject::Array(a) => a,
            _ => panic!(),
        }
    }
}



impl Index<usize> for JFObject {
    type Output = JFObject;
    fn index<'a>(&'a self, id: usize) -> &'a Self::Output {
        self.into_vec().unwrap().get(id).unwrap()
    }
}

impl Index<String> for JFObject {
    type Output = JFObject;
    fn index<'a>(&'a self, id: String) -> &'a Self::Output {
        self.into_hashmap().unwrap().get(&id).unwrap()
    }
}

impl<'a> Index<&'a str> for JFObject {
    type Output = JFObject;
    fn index<'b>(&'b self, id: &str) -> &'b Self::Output {
        self.into_hashmap().unwrap().get(&id.to_owned()).unwrap()
    }
}


fn recursive(v: &mut JFObject,
             a_chain: Vec<i64>,
             d_chain: Vec<String>,
             mut a_nest: i64,
             mut d_nest: i64,
             last_chain: char,
             last_c: char,
             func: fn(&mut JFObject,
                      Option<String>,
                      Vec<i64>,
                      Vec<String>,
                      i64,
                      i64,
                      char)
                     ,
             value: Option<String>,
             mut log: String)
             -> bool {

    if DEBUG {
        log = format!("{}{}",
                      log,
                      format!("--> [a_chain: {:?}, a_nest:{}  d_chain: {:?}, d_nest:{}] ",
                              a_chain,
                              a_nest,
                              d_chain,
                              d_nest)
                          .to_owned());
    }

    let is_find = match *v {

        JFObject::Array(ref mut vvz) => {
            let i = *a_chain.get(a_nest as usize).unwrap();
            let is_find: bool = {
                let vvv = vvz.get_mut(i as usize);
                let is_find: bool = match vvv {
                    Some(mut vvvv) => {
                        a_nest += 1;
                        recursive(&mut vvvv,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  value.clone(),
                                  log);
                        a_nest -= 1;
                        true
                    }
                    None => false,
                };
                is_find
            };
            if !is_find {
            }
            is_find
        }

        JFObject::Dictionary(ref mut vv) => {
            let o_key = d_chain.get(d_nest as usize);
            match o_key {
                Some(ref key) => {
                    let vvv = vv.get_mut(key.clone());
                    let is_find: bool = match vvv {
                        Some(mut vvvv) => {
                            d_nest += 1;
                            recursive(&mut vvvv,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      value.clone(),
                                      log);
                            d_nest -= 1;
                            true
                        }
                        None => false,
                    };
                    is_find
                }
                None => false,
            }
        }
        _ => true,
    };

    if !is_find {
        func(v,
             value,
             a_chain.clone(),
             d_chain.clone(),
             a_nest,
             d_nest,
             last_c);
    }
    is_find
}

pub fn decode(text: String) -> Box<JFObject> {

    let mut ret = Box::new(JFObject::Null);

    let mut pos: usize = 0;

    let mut chain: Vec<char> = Vec::new();
    let mut d_chain: Vec<String> = Vec::new();
    let mut a_chain: Vec<i64> = Vec::new();
    let mut last_chain: char = ' ';
    let mut last_active_char: char = ' ';
    let mut key: String;
    let mut string: String = "".to_owned();
    let mut num: String = "".to_owned();
    let mut last_c: char = ' ';
    let mut s_true: String = "".to_owned();
    let mut s_false: String = "".to_owned();
    let mut s_null: String = "".to_owned();

    let body: Vec<char> = text.chars().collect();
    let size = body.len();
    let mut done = false;
    while !done {

        let c: char = body[pos];

        match last_chain {
            's' => {
                string.push(c);
            }
            'w' => {
                string.push(c);
            }
            'n' => {
                num.push(c);
            }
            't' => {
                s_true.push(c);
            }
            'f' => {
                s_false.push(c);
            }
            '0' => {
                s_null.push(c);
            }
            _ => {}
        };

        if DEBUG {
            if c != ' ' {
                println!("\x1b[32mc: {}\t -- l: {}\t -- c: {:?}\t -- ac: {:?}\t -- dc: {:?}\t -- \
                          s: {}\t -- n: {} -- lac: {} -- t: {} -- f: {} -- 0: {}\x1b[0m",
                         body[pos],
                         last_chain,
                         chain,
                         a_chain,
                         d_chain,
                         string,
                         num,
                         last_active_char,
                         s_true,
                         s_false,
                         s_null);
            }
        }

        match c {

            '[' => {

                match last_chain {

                    's' => {}
                    'w' => {}

                    _ => {

                        let a = 'a';
                        chain.push(a);
                        last_chain = a;
                        a_chain.push(0);

                        let is_root = match *ret {

                            JFObject::Null => {
                                *ret = JFObject::Array(Vec::new());
                                true
                            }

                            _ => false,
                        };

                        if !is_root {
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::Array(Vec::new()));
                                    }
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        vv.insert(key, JFObject::Array(Vec::new()));
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);
                        }
                    }
                };
                last_active_char = c.clone();
            }

            ']' => {
                match last_chain {

                    's' => {}
                    'w' => {}
                    't' => {

                        s_true.pop().unwrap();
                        s_true = s_true.trim().to_string();
                        if s_true != "true" {
                            panic!("parse error");
                        }

                        let a_nest = 0i64;
                        let d_nest = 0i64;
                        let log: String = "".to_owned();
                        fn func(v: &mut JFObject,
                                _: Option<String>,
                                _: Vec<i64>,
                                _: Vec<String>,
                                _: i64,
                                _: i64,
                                _: char) {
                            match *v {
                                JFObject::Array(ref mut vv) => {
                                    vv.push(JFObject::True);
                                }
                                _ => {}
                            }
                        }
                        recursive(&mut ret,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  None,
                                  log);

                        chain.pop().unwrap();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                        a_chain.pop().unwrap();
                        s_true = "".to_owned();
                    }

                    'f' => {

                        s_false.pop().unwrap();
                        s_false = s_false.trim().to_string();
                        if s_false != "false" {
                            panic!("parse error");
                        }

                        let a_nest = 0i64;
                        let d_nest = 0i64;
                        let log: String = "".to_owned();
                        fn func(v: &mut JFObject,
                                _: Option<String>,
                                _: Vec<i64>,
                                _: Vec<String>,
                                _: i64,
                                _: i64,
                                _: char) {
                            match *v {
                                JFObject::Array(ref mut vv) => {
                                    vv.push(JFObject::False);
                                }
                                _ => {}
                            }
                        }
                        recursive(&mut ret,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  None,
                                  log);

                        chain.pop().unwrap();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                        a_chain.pop().unwrap();

                        s_false = "".to_owned();
                    }

                    '0' => {

                        s_null.pop().unwrap();
                        s_null = s_null.trim().to_string();
                        if s_null != "null" {
                            panic!("parse error");
                        }

                        let a_nest = 0i64;
                        let d_nest = 0i64;
                        let log: String = "".to_owned();
                        fn func(v: &mut JFObject,
                                _: Option<String>,
                                _: Vec<i64>,
                                _: Vec<String>,
                                _: i64,
                                _: i64,
                                _: char) {
                            match *v {
                                JFObject::Array(ref mut vv) => {
                                    vv.push(JFObject::Null);
                                }
                                _ => {}
                            }
                        }
                        recursive(&mut ret,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  None,
                                  log);


                        chain.pop().unwrap();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                        a_chain.pop().unwrap();

                        s_null = "".to_owned();
                    }

                    'n' => {

                        let a_nest = 0i64;
                        let d_nest = 0i64;
                        let log: String = "".to_owned();
                        fn func(v: &mut JFObject,
                                value: Option<String>,
                                _: Vec<i64>,
                                _: Vec<String>,
                                _: i64,
                                _: i64,
                                _: char) {
                            match *v {
                                JFObject::Array(ref mut vv) => {

                                    let mut new_num = value.unwrap().clone();
                                    new_num.pop().unwrap();
                                    new_num = new_num.trim().to_string();

                                    match new_num.find('.') {
                                        Some(_) => vv.push( JFObject::Float(f64::from_str(&new_num.clone()).unwrap()) ),
                                        None    => vv.push( JFObject::Integer(i64::from_str(&new_num.clone()).unwrap()) ),
                                    };
                                }
                                _ => {}
                            }
                        }
                        recursive(&mut ret,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  Some(num),
                                  log);

                        num = "".to_owned();

                        chain.pop().unwrap();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                        a_chain.pop().unwrap();

                    }

                    'a' => {


                        if last_active_char == ',' {

                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    _: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::Null);
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);

                        }

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                        a_chain.pop().unwrap();
                    }

                    _ => panic!("unknown chain from array"),
                }

                last_active_char = c.clone();

            }

            '{' => {

                match last_chain {

                    's' => {}
                    'w' => {}

                    'v' => {

                        let a = 'd';
                        chain.push(a);
                        last_chain = a;

                        let a_nest = 0i64;
                        let d_nest = 0i64;
                        let log: String = "".to_owned();

                        fn func(v: &mut JFObject,
                                _: Option<String>,
                                _: Vec<i64>,
                                d_chain: Vec<String>,
                                _: i64,
                                _: i64,
                                _: char) {
                            match *v {
                                JFObject::Array(ref mut vv) => {
                                    vv.push(JFObject::Dictionary(HashMap::new()));
                                }
                                JFObject::Dictionary(ref mut vv) => {
                                    let key = d_chain.last().unwrap().clone();
                                    vv.insert(key, JFObject::Dictionary(HashMap::new()));
                                }
                                _ => {}
                            }
                        }

                        recursive(&mut ret,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  None,
                                  log);
                    }

                    _ => {

                        let a = 'd';
                        chain.push(a);
                        last_chain = a;


                        let is_root = match *ret {
                            JFObject::Null => {
                                *ret = JFObject::Dictionary(HashMap::new());
                                true
                            }
                            _ => false,
                        };

                        if !is_root {
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::Dictionary(HashMap::new()));
                                    }
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        vv.insert(key, JFObject::Dictionary(HashMap::new()));
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);
                        }
                    }
                }

                last_active_char = c.clone();

            }

            '}' => {
                match last_chain {

                    's' => {}
                    'w' => {}

                    't' => {

                        s_true.pop().unwrap();
                        s_true = s_true.trim().to_string();
                        if s_true != "true" {
                            panic!("parse error");
                        }

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {

                                match *v {
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        vv.insert(key, JFObject::True);
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);
                        }

                        s_true = "".to_owned();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                    }

                    'f' => {

                        s_false.pop().unwrap();
                        s_false = s_false.trim().to_string();
                        if s_false != "false" {
                            panic!("parse error");
                        }

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {

                                match *v {
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        vv.insert(key, JFObject::False);
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);

                        }

                        s_false = "".to_owned();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                    }

                    '0' => {

                        s_null.pop().unwrap();
                        s_null = s_null.trim().to_string();
                        if s_null != "null" {
                            panic!("parse error");
                        }

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();


                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {

                                match *v {
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        vv.insert(key, JFObject::Null);
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);
                        }

                        s_null = "".to_owned();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                    }

                    'n' => {

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    value: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {

                                match *v {
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        let mut value = value.unwrap();
                                        value.pop().unwrap();
                                        value = value.trim().to_string();
                                        match value.find('.') {
                                            Some(_) => vv.insert(key, JFObject::Float(f64::from_str(&value.clone()).unwrap()) ),
                                            None    => vv.insert(key, JFObject::Integer(i64::from_str(&value.clone()).unwrap()) ),
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      Some(num.clone()),
                                      log);

                        }
                        num = "".to_owned();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                    }

                    'v' => {
                        d_chain.pop().unwrap();
                        chain.pop().unwrap();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                    }

                    _ => {
                        d_chain.pop().unwrap();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                    }
                }
                last_active_char = c.clone();
            }

            ':' => {
                match last_chain {

                    's' => {}
                    'w' => {}

                    'd' => {

                        let v = 'v';
                        chain.push(v);
                        last_chain = v;

                        key = string.clone();
                        key.pop().unwrap();

                        d_chain.push(key.clone());

                        string = "".to_owned();
                    }

                    _ => {}
                }

                last_active_char = c.clone();

            }

            ',' => {
                match last_chain {

                    's' => {}
                    'w' => {}

                    't' => {

                        s_true.pop().unwrap();
                        s_true = s_true.trim().to_string();
                        if s_true != "true" {
                            panic!("parse error");
                        }

                        if last_chain == 't' {

                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::True);
                                    }
                                    JFObject::Dictionary(ref mut vv) => {

                                        let key = d_chain.last().unwrap().clone();

                                        vv.insert(key, JFObject::True);

                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);

                        }

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            last_chain = chain.last().unwrap_or(&' ').to_owned();
                            d_chain.pop().unwrap();
                        } else {
                            let a = a_chain.pop().unwrap();
                            a_chain.push(a + 1i64);
                        }

                        s_true = "".to_owned();
                    }

                    'f' => {

                        s_false.pop().unwrap();
                        s_false = s_false.trim().to_string();
                        if s_false != "false" {
                            panic!("parse error");
                        }

                        if last_chain == 'f' {
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::False);
                                    }
                                    JFObject::Dictionary(ref mut vv) => {

                                        let key = d_chain.last().unwrap().clone();

                                        vv.insert(key, JFObject::False);

                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);

                        }

                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            last_chain = chain.last().unwrap_or(&' ').to_owned();
                            d_chain.pop().unwrap();
                        } else {
                            let a = a_chain.pop().unwrap();
                            a_chain.push(a + 1i64);
                        }

                        s_false = "".to_owned();
                    }

                    '0' => {

                        s_null.pop().unwrap();
                        s_null = s_null.trim().to_string();
                        if s_null != "null" {
                            panic!("parse error");
                        }

                        if last_chain == '0' {
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    d_chain: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::Null);
                                    }
                                    JFObject::Dictionary(ref mut vv) => {
                                        let key = d_chain.last().unwrap().clone();
                                        vv.insert(key, JFObject::Null);
                                    }
                                    _ => {}
                                }
                            }
                            chain.pop().unwrap();
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);

                        }

                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            last_chain = chain.last().unwrap_or(&' ').to_owned();
                            d_chain.pop().unwrap();
                        } else {
                            let a = a_chain.pop().unwrap();
                            a_chain.push(a + 1i64);
                        }
                        s_null = "".to_owned();
                    }

                    'a' => {
                        let a = a_chain.pop().unwrap();
                        a_chain.push(a + 1i64);
                        if last_active_char == '[' || last_active_char == ',' {
                            let a_nest = 0i64;
                            let d_nest = 0i64;
                            let log: String = "".to_owned();
                            fn func(v: &mut JFObject,
                                    _: Option<String>,
                                    _: Vec<i64>,
                                    _: Vec<String>,
                                    _: i64,
                                    _: i64,
                                    _: char) {
                                match *v {
                                    JFObject::Array(ref mut vv) => {
                                        vv.push(JFObject::Null);
                                    }
                                    _ => {}
                                }
                            }
                            recursive(&mut ret,
                                      a_chain.clone(),
                                      d_chain.clone(),
                                      a_nest,
                                      d_nest,
                                      last_chain,
                                      last_c,
                                      func,
                                      None,
                                      log);
                        }
                    }

                    'n' => {

                        let a_nest = 0i64;
                        let d_nest = 0i64;
                        let log: String = "".to_owned();
                        fn func(v: &mut JFObject,
                                value: Option<String>,
                                _: Vec<i64>,
                                d_chain: Vec<String>,
                                _: i64,
                                _: i64,
                                _: char) {
                            match *v {
                                JFObject::Array(ref mut vv) => {
                                    let mut new_num = value.unwrap().clone();
                                    new_num.pop().unwrap();
                                    new_num = new_num.trim().to_string();

                                    match new_num.find('.') {
                                        Some(_) => {
                                            vv.push(JFObject::Float(f64::from_str(&new_num)
                                                                        .unwrap()))
                                        }
                                        None => {
                                            vv.push(JFObject::Integer(i64::from_str(&new_num)
                                                                          .unwrap()))
                                        }
                                    };

                                }
                                JFObject::Dictionary(ref mut vv) => {

                                    let key = d_chain.last().unwrap().clone();

                                    let mut new_num = value.unwrap().clone();
                                    new_num.pop().unwrap();
                                    new_num = new_num.trim().to_string();

                                    match new_num.find('.') {
                                        Some(_) => {
                                            vv.insert(key,
                                                      JFObject::Float(f64::from_str(&new_num)
                                                                          .unwrap()))
                                        }
                                        None => {
                                            vv.insert(key,
                                                      JFObject::Integer(i64::from_str(&new_num)
                                                                            .unwrap()))
                                        }
                                    };


                                }
                                _ => {}
                            }
                        }
                        recursive(&mut ret,
                                  a_chain.clone(),
                                  d_chain.clone(),
                                  a_nest,
                                  d_nest,
                                  last_chain,
                                  last_c,
                                  func,
                                  Some(num),
                                  log);

                        num = "".to_owned();
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();

                        if last_chain == 'v' {
                            chain.pop().unwrap();
                            last_chain = chain.last().unwrap_or(&' ').to_owned();
                            d_chain.pop().unwrap();
                        } else {
                            let a = a_chain.pop().unwrap();
                            a_chain.push(a + 1i64);
                        }

                    }

                    'v' => {
                        chain.pop().unwrap();
                        last_chain = chain.last().unwrap_or(&' ').to_owned();
                        d_chain.pop().unwrap();
                    }
                    _ => {}
                }

                last_active_char = c.clone();

            }

            '"' => {


                match last_chain {

                    'w' => {
                        if last_c != '\\' {

                            chain.pop().unwrap();
                            last_chain = chain.last().unwrap_or(&' ').to_owned();

                            if last_chain == 'v' {

                                let a_nest = 0i64;
                                let d_nest = 0i64;
                                let log: String = "".to_owned();
                                fn func(v: &mut JFObject,
                                        value: Option<String>,
                                        _: Vec<i64>,
                                        d_chain: Vec<String>,
                                        _: i64,
                                        _: i64,
                                        _: char) {

                                    match *v {
                                        JFObject::Dictionary(ref mut vv) => {
                                            let key = d_chain.last().unwrap().clone();
                                            let mut value = value.unwrap();
                                            value.pop().unwrap();
                                            vv.insert(key, JFObject::String(value.clone()));
                                        }
                                        _ => {}
                                    }
                                }
                                recursive(&mut ret,
                                          a_chain.clone(),
                                          d_chain.clone(),
                                          a_nest,
                                          d_nest,
                                          last_chain,
                                          last_c,
                                          func,
                                          Some(string.clone()),
                                          log);
                                string = "".to_owned();
                            } else if last_chain != 'd' {
                                string.pop().unwrap();
                                let is_root = match *ret {
                                    JFObject::Null => {
                                        *ret = JFObject::String(string.clone());
                                        true
                                    }
                                    _ => false,
                                };

                                if !is_root {
                                    let a_nest = 0i64;
                                    let d_nest = 0i64;
                                    let log: String = "".to_owned();
                                    fn func(v: &mut JFObject,
                                            value: Option<String>,
                                            _: Vec<i64>,
                                            _: Vec<String>,
                                            _: i64,
                                            _: i64,
                                            _: char) {
                                        match *v {
                                            JFObject::Array(ref mut vv) => {
                                                vv.push(JFObject::String(value.unwrap()
                                                                              .clone()));
                                            }
                                            _ => {}
                                        }
                                    }
                                    recursive(&mut ret,
                                              a_chain.clone(),
                                              d_chain.clone(),
                                              a_nest,
                                              d_nest,
                                              last_chain,
                                              last_c,
                                              func,
                                              Some(string),
                                              log);
                                }
                                string = "".to_owned();
                            }
                        }
                    }

                    _ => {
                        let w = 'w';
                        chain.push(w);
                        last_chain = w;
                        string = "".to_owned();
                    }
                }

                last_active_char = c.clone();

            }
            '\'' => {
                match last_chain {
                    's' => {
                        if last_c != '\\' {

                            chain.pop().unwrap();
                            last_chain = chain.last().unwrap_or(&' ').to_owned();

                            if last_chain == 'v' {
                                let a_nest = 0i64;
                                let d_nest = 0i64;
                                let log: String = "".to_owned();
                                fn func(v: &mut JFObject,
                                        value: Option<String>,
                                        _: Vec<i64>,
                                        d_chain: Vec<String>,
                                        _: i64,
                                        _: i64,
                                        _: char) {

                                    match *v {
                                        JFObject::Dictionary(ref mut vv) => {
                                            let key = d_chain.last().unwrap().clone();
                                            let mut value = value.unwrap();
                                            value.pop().unwrap();
                                            vv.insert(key, JFObject::String(value.clone()));
                                        }
                                        _ => {}
                                    }
                                }
                                recursive(&mut ret,
                                          a_chain.clone(),
                                          d_chain.clone(),
                                          a_nest,
                                          d_nest,
                                          last_chain,
                                          last_c,
                                          func,
                                          Some(string.clone()),
                                          log);
                                d_chain.pop().unwrap();
                                string = "".to_owned();
                            } else {
                                string.pop().unwrap();
                                let is_root = match *ret {
                                    JFObject::Null => {
                                        *ret = JFObject::String(string.clone());
                                        true
                                    }
                                    _ => false,
                                };

                                if !is_root {
                                    let a_nest = 0i64;
                                    let d_nest = 0i64;
                                    let log: String = "".to_owned();
                                    fn func(v: &mut JFObject,
                                            value: Option<String>,
                                            _: Vec<i64>,
                                            _: Vec<String>,
                                            _: i64,
                                            _: i64,
                                            _: char) {
                                        match *v {
                                            JFObject::Array(ref mut vv) => {
                                                vv.push(JFObject::String(value.unwrap()
                                                                              .clone()));
                                            }
                                            _ => {}
                                        }
                                    }
                                    recursive(&mut ret,
                                              a_chain.clone(),
                                              d_chain.clone(),
                                              a_nest,
                                              d_nest,
                                              last_chain,
                                              last_c,
                                              func,
                                              Some(string),
                                              log);
                                }
                                string = "".to_owned();
                            }
                        }
                    }
                    _ => {
                        string = "".to_owned();
                        let s = 's';
                        chain.push(s);
                        last_chain = s;
                    }
                }
                last_active_char = c.clone();
            }

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                match last_chain {
                    'n' => {}
                    'w' => {}
                    's' => {}
                    _ => {
                        num = "".to_owned();
                        let n = 'n';
                        chain.push(n);
                        last_chain = n;
                        num.push(c);
                    }
                }
                last_active_char = c.clone();
            }

            '-' => {
                match last_chain {
                    'n' => {}
                    'w' => {}
                    's' => {}
                    _ => {
                        num = "".to_owned();
                        let n = 'n';
                        chain.push(n);
                        last_chain = n;
                        num.push(c);
                    }
                }
                last_active_char = c.clone();
            }

            't' => {

                match last_chain {
                    'n' => {}
                    'w' => {}
                    's' => {}

                    _ => {
                        let t = 't';
                        chain.push(t);
                        last_chain = t;
                        s_true = "".to_owned();
                        s_true.push(c);
                    }
                }
                last_active_char = c.clone();
            }

            'f' => {
                match last_chain {
                    'n' => {}
                    'w' => {}
                    's' => {}
                    _ => {
                        let f = 'f';
                        chain.push(f);
                        last_chain = f;
                        s_false = "".to_owned();
                        s_false.push(c);
                    }
                }
                last_active_char = c.clone();
            }

            'n' => {
                match last_chain {
                    'n' => {}
                    'w' => {}
                    's' => {}
                    _ => {
                        let null = '0';
                        chain.push(null);
                        last_chain = null;
                        s_null = "".to_owned();
                        s_null.push(c);
                    }
                }
                last_active_char = c.clone();
            }

            '\n' => {}
            _ => {}
        };

        if DEBUG {
            if c != ' ' {
                println!("\x1b[35mc: {}\t -- l: {}\t -- c: {:?}\t -- ac: {:?}\t -- dc: {:?}\t -- \
                          s: {}\t -- n: {} -- lac: {} -- t: {} -- f: {} -- 0: {}\x1b[0m",
                         body[pos],
                         last_chain,
                         chain,
                         a_chain,
                         d_chain,
                         string,
                         num,
                         last_active_char,
                         s_true,
                         s_false,
                         s_null);
            }
        }

        pos += 1;
        if pos >= size {
            done = true;
        }

        last_c = c.clone();

    }


    ret
}
