use json_flex::{JsonFlex, JFObject, Unwrap};
use std::collections::HashMap;

#[test]
fn test () {

    /* 1
    -------------------------------------------------------------------------------*/
    println!("--- [ 1 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["a", "b", "c", ["a", "b", "c"], "d", ["ABC"],[1,2]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([String("a"), String("b"), String("c"), Array([String("a"), String("b"), String("c")]), String("d"), Array([String("ABC")]), Array([Integer(1), Integer(2)])])"#.to_owned());

    /* 2
    -------------------------------------------------------------------------------*/
    println!("--- [ 2 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,[2,[3]]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Integer(1), Array([Integer(2), Array([Integer(3)])])])"#.to_owned());

    /* 3
    -------------------------------------------------------------------------------*/
    println!("--- [ 3 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,[2,[3,4,5,6]]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Integer(1), Array([Integer(2), Array([Integer(3), Integer(4), Integer(5), Integer(6)])])])"#.to_owned());

    /* 4
    -------------------------------------------------------------------------------*/
    println!("--- [ 4 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["1",["2",["3","4","5","6"]]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([String("1"), Array([String("2"), Array([String("3"), String("4"), String("5"), String("6")])])])"#.to_owned());

    /* 5                           [[1],[2,3,4,[5,6,7,[8,9],11],12,13],14,15]
    -------------------------------------------------------------------------------*/
    println!("--- [ 5 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[[1],[2,3,4,[5,6,7,[8,9],11],12,13],14,15]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Array([Integer(1)]), Array([Integer(2), Integer(3), Integer(4), Array([Integer(5), Integer(6), Integer(7), Array([Integer(8), Integer(9)]), Integer(11)]), Integer(12), Integer(13)]), Integer(14), Integer(15)])"#.to_owned());

    /* 6
    -------------------------------------------------------------------------------*/
    println!("--- [ 6 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"['a', 'b', 'c', ['a', 'b', 'c'], 'd', ['ABC'],[1,2]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([String("a"), String("b"), String("c"), Array([String("a"), String("b"), String("c")]), String("d"), Array([String("ABC")]), Array([Integer(1), Integer(2)])])"#.to_owned());

    /* 7
    -------------------------------------------------------------------------------*/
    println!("--- [ 7 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc": "def"}]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": String("def")})])"#);

    /* 8
    -------------------------------------------------------------------------------*/
    println!("--- [ 8 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc": 123}]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": Integer(123)})])"#);

    /* 9
    -------------------------------------------------------------------------------*/
    println!("--- [ 9 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc": [1]}]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": Array([Integer(1)])})])"#);

    /* 10
    -------------------------------------------------------------------------------*/
    println!("--- [ 10 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc": ["1"]}]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": Array([String("1")])})])"#);

    /* 12
    -------------------------------------------------------------------------------*/
    println!("--- [ 12 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc": { "def": "ghi" } }]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": Dictionary({"def": String("ghi")})})])"#);

    /* 13
    -------------------------------------------------------------------------------*/
    println!("--- [ 13 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc": { "def": ["1","2","3"] } }]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": Dictionary({"def": Array([String("1"), String("2"), String("3")])})})])"#);

    /* 14
    -------------------------------------------------------------------------------*/
    println!("--- [ 14 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc":"def", "ghi": "jkl" }]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": String("def"), "ghi": String("jkl")})])"# || jft == r#"Array([Dictionary({"ghi": String("jkl"), "abc": String("def")})])"#);

    /* 15
    -------------------------------------------------------------------------------*/
    println!("--- [ 15 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc":123, "def": 456 }]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"def": Integer(456), "abc": Integer(123)})])"# || jft == r#"Array([Dictionary({"abc": Integer(123), "def": Integer(456)})])"#);

    /* 16
    -------------------------------------------------------------------------------*/
    println!("--- [ 16 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[{"abc":123, "def": { "ghi": [1,2,3, { "ssss": "ssssss" }] } }]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Dictionary({"abc": Integer(123), "def": Dictionary({"ghi": Array([Integer(1), Integer(2), Integer(3), Dictionary({"ssss": String("ssssss")})])})})])"# || jft == r#"Array([Dictionary({"def": Dictionary({"ghi": Array([Integer(1), Integer(2), Integer(3), Dictionary({"ssss": String("ssssss")})])}), "abc": Integer(123)})])"#);

    /* 17 (Google JSON)
    -------------------------------------------------------------------------------*/
    println!("--- [ 17 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[,"foo foo bar",[],["URL","NAME"],[]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Null, String("foo foo bar"), Array([]), Array([String("URL"), String("NAME")]), Array([])])"#);

    /* 18 (Google JSON)
    -------------------------------------------------------------------------------*/
    println!("--- [ 18 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[,,,,,,,,"foo foo bar",[],["URL","NAME"],[]]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Null, Null, Null, Null, Null, Null, Null, Null, String("foo foo bar"), Array([]), Array([String("URL"), String("NAME")]), Array([])])"#);

    /* 19 (Google JSON)
    -------------------------------------------------------------------------------*/
    println!("--- [ 19 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["foo foo bar",[],["URL","NAME"],[],]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([String("foo foo bar"), Array([]), Array([String("URL"), String("NAME")]), Array([]), Null])"#);

    /* 20 (Google JSON)
    -------------------------------------------------------------------------------*/
    println!("--- [ 20 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["foo foo bar",[],["URL","NAME"],[],,,,,,,,,,,,,,,,,,]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([String("foo foo bar"), Array([]), Array([String("URL"), String("NAME")]), Array([]), Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null, Null])"#);


    /* 21 (Google JSON)
    -------------------------------------------------------------------------------*/
    println!("--- [ 21 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[,]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Null, Null])"#);

    /* 22
    -------------------------------------------------------------------------------*/
    println!("--- [ 22 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[-100]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Integer(-100)])"#);

    /* 23
    -------------------------------------------------------------------------------*/
    println!("--- [ 23 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[-100, -200, -300]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Integer(-100), Integer(-200), Integer(-300)])"#);

    /* 24
    -------------------------------------------------------------------------------*/
    println!("--- [ 24 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": -100}"#.to_owned());
    let jft = format!("{:?}", jf);
    assert!(jft == r#"Dictionary({"foo": Integer(-100)})"#);

    /* 25
    -------------------------------------------------------------------------------*/
    println!("--- [ 25 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": [1,2,3] }"#.to_owned());
    let jft = format!("{:?}", jf);
    assert!(jft == r#"Dictionary({"foo": Array([Integer(1), Integer(2), Integer(3)])})"#);

    /* 26
    -------------------------------------------------------------------------------*/
    println!("--- [ 26 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": {"baz": 1, "fum": "2" } }"#.to_owned());
    let jft = format!("{:?}", jf);
    assert!(jft == r#"Dictionary({"foo": Dictionary({"baz": Integer(1), "fum": String("2")})})"# || jft == r#"Dictionary({"foo": Dictionary({"fum": String("2"), "baz": Integer(1)})})"#);


    /* 27
    -------------------------------------------------------------------------------*/
    println!("--- [ 27 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[true]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([True])"#);

    /* 28
    -------------------------------------------------------------------------------*/
    println!("--- [ 28 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[true,true]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([True, True])"#);

    /* 29
    -------------------------------------------------------------------------------*/
    println!("--- [ 29 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[false]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([False])"#);

    /* 30
    -------------------------------------------------------------------------------*/
    println!("--- [ 30 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[false,false]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([False, False])"#);

    /* 31
    -------------------------------------------------------------------------------*/
    println!("--- [ 31 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": true}"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Dictionary({"foo": True})"#);

    /* 32
    -------------------------------------------------------------------------------*/
    println!("--- [ 32 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": false}"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Dictionary({"foo": False})"#);

    /* 33
    -------------------------------------------------------------------------------*/
    println!("--- [ 33 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": true, "baz": false}"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Dictionary({"foo": True, "baz": False})"# || jft == r#"Dictionary({"baz": False, "foo": True})"#);

    /* 34
    -------------------------------------------------------------------------------*/
    println!("--- [ 34 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": false, "baz": true}"#.to_owned());
    let jft = format!("{:?}", jf);
    assert!(jft == r#"Dictionary({"baz": True, "foo": False})"# || jft == r#"Dictionary({"foo": False, "baz": True})"#);

    /* 35
    -------------------------------------------------------------------------------*/
    println!("--- [ 35 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[null]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Null])"#);

    /* 36
    -------------------------------------------------------------------------------*/
    println!("--- [ 36 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[null,null]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Array([Null, Null])"#);

    /* 37
    -------------------------------------------------------------------------------*/
    println!("--- [ 37 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": null}"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Dictionary({"foo": Null})"#);

    /* 38
    -------------------------------------------------------------------------------*/
    println!("--- [ 38 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": null, "baz": null}"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert!(jft == r#"Dictionary({"baz": Null, "foo": Null})"# || jft == r#"Dictionary({"foo": Null, "baz": Null})"#);


    /* 39
    -------------------------------------------------------------------------------*/
    println!("--- [ 39 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{"foo": null, "baz": null, "fuge": [1,,,2,,3,,4,,null,null,null,,,,5,6,"7"]}"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);

    /* 40
    -------------------------------------------------------------------------------*/
    println!("--- [ 40 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[,[[],],{"false":0,"true":1}]﻿"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    
    /* 41
    -------------------------------------------------------------------------------*/
    println!("--- [ 41 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2,3, { "foo": [4,5,6] } ]"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    
    /* 42
    -------------------------------------------------------------------------------*/
    println!("--- [ 42 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2,3, { "foo": [4,5,6] } ]"#.to_owned());
    let jft = format!("{:?}", jf[0]);
    println!("{}", jft);
    let jft = format!("{:?}", jf[1]);
    println!("{}", jft);
    let jft = format!("{:?}", jf[2]);
    println!("{}", jft);
    let jft = format!("{:?}", jf[3]);
    println!("{}", jft);
    let jft = format!("{:?}", jf[3]["foo"][0]);
    println!("{}", jft);
    assert!(jft == r#"Integer(4)"#);
    
    /* 43
    -------------------------------------------------------------------------------*/
    println!("--- [ 43 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2,2.5]﻿"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    
    /* 44
    -------------------------------------------------------------------------------*/
    println!("--- [ 44 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2,2.5,4]﻿"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    
    /* 45
    -------------------------------------------------------------------------------*/
    println!("--- [ 45 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2,2.5,4,{"foo":1.2}]﻿"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    
    /* 46
    -------------------------------------------------------------------------------*/
    println!("--- [ 46 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2,2.5,4,{"foo":1.2, "baz": [1,2,4.5]}]﻿"#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    
    /* 47
    -------------------------------------------------------------------------------*/
    println!("--- [ 47 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["1", "2"]﻿"#.to_owned());
    let foo: String = jf[0].clone().unwrap();
    let jft = format!("{:?}", foo);
    println!("{}", jft);
    assert!(jft == r#""1""#);
    
    /* 48
    -------------------------------------------------------------------------------*/
    println!("--- [ 48 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["1", "2"]﻿"#.to_owned());
    let foo: JFObject = jf[0].clone();
    let jft = format!("{:?}", foo);
    println!("{}", jft);
    assert!(jft == r#"String("1")"#);
    
    /* 49
    -------------------------------------------------------------------------------*/
    println!("--- [ 49 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"
        {
           "Accept-Language": "en-US,en;q=0.8",
           "Host": "example.com",
           "Accept-Charset": "ISO-8859-1,utf-8;q=0.7,*;q=0.3",
           "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
        }
    "#.to_owned());
    let jft = format!("{:?}", jf);
    println!("{}", jft);
    assert_eq!(jf["Accept-Language"].unwrap_string(), "en-US,en;q=0.8");
    assert_eq!(jf["Host"].unwrap_string(), "example.com");
    assert_eq!(jf["Accept-Charset"].unwrap_string(), "ISO-8859-1,utf-8;q=0.7,*;q=0.3");
    assert_eq!(jf["Accept"].unwrap_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8");
    
    /* 50
    -------------------------------------------------------------------------------*/
    println!("--- [ 50 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["1", "2"]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].into_string());
    println!("{}", jft);
    assert_eq!(jft, r#"Some("1")"#);
    
    /* 51
    -------------------------------------------------------------------------------*/
    println!("--- [ 51 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1, 2]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].into_i64());
    println!("{}", jft);
    assert_eq!(jft, r#"Some(1)"#);
    
    /* 52
    -------------------------------------------------------------------------------*/
    println!("--- [ 52 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1.1, 2.2]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].into_f64());
    println!("{}", jft);
    assert_eq!(jft, r#"Some(1.1)"#);
    
    /* 53
    -------------------------------------------------------------------------------*/
    println!("--- [ 53 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{ "foo": "bar" }﻿"#.to_owned());
    let jft = format!("{:?}", jf.into_hashmap());
    println!("{}", jft);
    assert_eq!(jft, r#"Some({"foo": String("bar")})"#);
    
    /* 54
    -------------------------------------------------------------------------------*/
    println!("--- [ 54 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1, 2]﻿"#.to_owned());
    let jft = format!("{:?}", jf.into_vec());
    println!("{}", jft);
    assert_eq!(jft, r#"Some([Integer(1), Integer(2)])"#);
    
    /* 55
    -------------------------------------------------------------------------------*/
    println!("--- [ 55 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[null, null]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].is_null());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 56
    -------------------------------------------------------------------------------*/
    println!("--- [ 56 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[true, true]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].is_true());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 57
    -------------------------------------------------------------------------------*/
    println!("--- [ 57 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[false, false]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].is_false());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 58
    -------------------------------------------------------------------------------*/
    println!("--- [ 58 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1, 2]﻿"#.to_owned());
    let jft = format!("{:?}", jf.is_array());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 59
    -------------------------------------------------------------------------------*/
    println!("--- [ 59 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{ "foo": "bar" }﻿"#.to_owned());
    let jft = format!("{:?}", jf.is_dictionary());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 60
    -------------------------------------------------------------------------------*/
    println!("--- [ 60 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["ABC", "DEF"]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].is_string());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 61
    -------------------------------------------------------------------------------*/
    println!("--- [ 61 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1, 2]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].is_integer());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 62
    -------------------------------------------------------------------------------*/
    println!("--- [ 62 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1.1, 2.2]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].is_float());
    println!("{}", jft);
    assert_eq!(jft, r#"true"#);
    
    /* 63
    -------------------------------------------------------------------------------*/
    println!("--- [ 63 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["ABC", "DEF"]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].unwrap_string());
    println!("{}", jft);
    assert_eq!(jft, r#""ABC""#);
    
    /* 64
    -------------------------------------------------------------------------------*/
    println!("--- [ 64 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].unwrap_i64());
    println!("{}", jft);
    assert_eq!(jft, r#"1"#);
    
    /* 65
    -------------------------------------------------------------------------------*/
    println!("--- [ 65 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1.1,2.2]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0].unwrap_f64());
    println!("{}", jft);
    assert_eq!(jft, r#"1.1"#);
    
    /* 66
    -------------------------------------------------------------------------------*/
    println!("--- [ 66 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{ "foo": "bar" }﻿"#.to_owned());
    let jft = format!("{:?}", jf.unwrap_hashmap());
    println!("{}", jft);
    assert_eq!(jft, r#"{"foo": String("bar")}"#);
    
    /* 67
    -------------------------------------------------------------------------------*/
    println!("--- [ 67 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1, 2]﻿"#.to_owned());
    let jft = format!("{:?}", jf.unwrap_vec());
    println!("{}", jft);
    assert_eq!(jft, r#"[Integer(1), Integer(2)]"#);
    
    /* 68
    -------------------------------------------------------------------------------*/
    println!("--- [ 68 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"["ABC", "DEF"]﻿"#.to_owned());
    let string: String = jf[0].clone().unwrap();
    let jft = format!("{:?}", string);
    println!("{}", jft);
    assert_eq!(jft, r#""ABC""#);
    
    /* 69
    -------------------------------------------------------------------------------*/
    println!("--- [ 69 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2]﻿"#.to_owned());
    let integer: i64 = jf[0].clone().unwrap();
    let jft = format!("{:?}", integer);
    println!("{}", jft);
    assert_eq!(jft, r#"1"#);
    
    /* 70
    -------------------------------------------------------------------------------*/
    println!("--- [ 70 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1.1,2.2]﻿"#.to_owned());
    let float: f64 = jf[0].clone().unwrap();
    let jft = format!("{:?}", float);
    println!("{}", jft);
    assert_eq!(jft, r#"1.1"#);
    
    /* 71
    -------------------------------------------------------------------------------*/
    println!("--- [ 71 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{ "foo": "bar" }﻿"#.to_owned());
    let hashmap: HashMap<String, JFObject> = jf.clone().unwrap();
    let jft = format!("{:?}", hashmap);
    println!("{}", jft);
    assert_eq!(jft, r#"{"foo": String("bar")}"#);
    
    /* 73
    -------------------------------------------------------------------------------*/
    println!("--- [ 73 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[1,2]﻿"#.to_owned());
    let vec: Vec<JFObject> = jf.clone().unwrap();
    let jft = format!("{:?}", vec);
    println!("{}", jft);
    assert_eq!(jft, r#"[Integer(1), Integer(2)]"#);
    
    /* 74
    -------------------------------------------------------------------------------*/
    println!("--- [ 74 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"[[[[[[[1]]]]]]]﻿"#.to_owned());
    let jft = format!("{:?}", jf[0][0][0][0][0][0][0]);
    println!("{}", jft);
    assert_eq!(jft, r#"Integer(1)"#);
    
    /* 75
    -------------------------------------------------------------------------------*/
    println!("--- [ 75 ] -----------------------------------------------------------------");
    let jf = JsonFlex::decode(r#"{ "foo": "bar" }﻿"#.to_owned());
    let jft = format!("{:?}", jf["foo"]);
    println!("{}", jft);
    assert_eq!(jft, r#"String("bar")"#);
    
    
    
    

}