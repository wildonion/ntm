


use near_sdk::*;



// ------------------------------ utility macros
// -------------------------------------------------------------------------------------------------
// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

// https://doc.rust-lang.org/reference/procedural-macros.html
// https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/
// https://dev.to/dandyvica/rust-procedural-macros-step-by-step-tutorial-36n8
// https://doc.rust-lang.org/rust-by-example/macros.html
// https://doc.rust-lang.org/book/ch19-06-macros.html
// https://doc.rust-lang.org/reference/procedural-macros.html
// https://danielkeep.github.io/practical-intro-to-macros.html
// https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/
// https://www.youtube.com/watch?v=j0zDvzQr7WE&ab_channel=CodingTech
// https://steveklabnik.com/writing/an-overview-of-macros-in-rust
// https://hub.packtpub.com/creating-macros-in-rust-tutorial/
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/macros.html
// https://blog.logrocket.com/procedural-macros-in-rust/
// https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
// https://lib.rs/development-tools/procedural-macro-helpers
  
//// ➔ trait based proc macro attribute:
////    derive like macros like #[derive(Serialize, Deserialize, Copy, Clone, Debug)] to bound a struct to the trait which delegates trait implementation for the struct which contains the trait methods that will extend the interface of the type
////    cutom like macros like #[custom(out="code.wasm"] or #[near_bindgen] #[cfg(os)] or #[deprecated()] on top of struct or enum fields
////    convert a trait into a module that will extend the trait methods like near #[ex_contract(contract_name)] proc macro 
////    #[..] applies an attribute to the thing after it (struct, struct fields or crate) and  #![..] applies an attribute to the containing thing or crate
////    TokenStream arg using proc_macro2 crate and proc-macro = true flag inside the lib.rs file by using #[proc_macro], #[proc_macro_attribute] and #[proc_macro_derive] attributes
////    build #[payable] proc macro for rafael runtime the deposit() method 
// ...


/*

    item      ➔ an Item | an item, like a function, struct, module, etc.
    block     ➔ a BlockExpression | a block (i.e. a block of statements and/or an expression, surrounded by braces)
    stmt      ➔ a Statement without the trailing semicolon (except for item statements that require semicolons)
    pat_param ➔ a PatternNoTopAlt
    pat       ➔ at least any PatternNoTopAlt, and possibly more depending on edition
    expr      ➔ an Expression
    ty        ➔ a Type
    ident     ➔ an IDENTIFIER_OR_KEYWORD or RAW_IDENTIFIER
    path      ➔ a TypePath style path | a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
    tt        ➔ a TokenTree (a single token or tokens in matching delimiters (), [], or {})
    meta      ➔ an Attr, the contents of an attribute | a meta item; the things that go inside #[...] and #![...] attributes
    lifetime  ➔ a LIFETIME_TOKEN
    vis       ➔ a possibly empty Visibility qualifier
    literal   ➔ matches -?LiteralExpression


//// ------------------------------------------------------------------------------------
//// macro based for pub/sub streamer actor on top of tokio, zmq and libp2p sockets like:
//// ------------------------------------------------------------------------------------
let stream = streamer!{
    tlp: "p2p|zmq|rpc|tokio-tcp|tokio-udp|local"
    type: "pub/sub"
    publish{
        topic: ""
        at: ""
    }
}

let res = streamer!{
    tlp: "p2p|zmq|rpc|tokio-tcp|tokio-udp|local"
    type: "pub/sub"
    subscribe{
        topic: ""

    }
}


//// ------------------------------
//// event and contract macros like
//// ------------------------------
TODO - simply emit the event when something just happens like this: emit!(list_owner)

contract!{


    name: "rev",
    args: [contract_owner, deposit_by_owner, contract_balance],

    // contract methods

    fn get_all_deposits(){

    }

    fn get_all_owners(){

    }


}

sign!{
    contract_name
}

event!{

    name: "list_owner",
    log: [NewOwner, AddDeposit],

    // event methods

    fn add_owner(){

    } 

    fn add_deposit(){
        
    }


}

emit!{
    event_name
}

//// ------------------------
//// event manager macro
//// ------------------------

list![func1, func2, func3] => {
    // first run event func2 then func1 and finally func3
    func2();
    func1();
    func3();
};


*/


#[macro_export]
macro_rules! list {
    ($id1:ident | $id2:ident <- [$start:expr; $end:expr], $cond:expr) => { //// the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc
        { //.... code block to return vec since using let statements must be inside {} block
            let mut vec = Vec::new();
            for num in $start..$end + 1{
                if $cond(num){
                    vec.push(num);
                }
            }
            vec
        } //....
    };
}
//////
/// let even = |x: i32| x%2 == 0;
/// let odd = |x: i32| x%2 != 0;
/// let evens = list![x | x <- [1; 10], even];
//////

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr)*) => { //// if this pattern matches the input the following code will be executed - * means we can pass more than one key => value statement
        { //.... code block to return vec since using let statements must be inside {} block
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )* //// * means we're inserting multiple key => value statement inside the map 
            map
        } //....
    };
}
//////
/// let d = dict!{"wildonion" => 1, "another_wildonion" => 2, "array": vec![1,3,4235,], "age": 24};
//////

#[macro_export]
macro_rules! exam {
    ($l:expr; and $r:expr) => { //// logical and match 
        $crate::macros::even(); //// calling even() function which is inside the macros module
        println!("{}", $l && $r);
    };

    ($l:expr; or $r:expr) => { //// logical or match 
        println!("{}", $l || $r);
    };
}
//////
/// exam!(1 == 2; and 3 == 2+1)
/// exam!(1 == 2; or 3 == 2+1)
//////


#[macro_export]
macro_rules! gendeh {
    ($iden:ident, $ty: tt) => {
        pub struct $iden(pub $ty);
        impl Default for $iden{
            fn default() -> Self{
                todo!()
            }
        }  
    };

    ($func_name:ident) => {
        fn $func_name(){
            println!("you've just called {:?}()", stringify!($func_name));
        }
    }
}
//////
/// gendeh!{bindgen, id} //// bindgen is the name of the struct and id is the name of the field
//////


#[macro_export]
macro_rules! query { // NOTE - this is a macro with multiple syntax support and if any pattern matches with the caller pattern, then the code block of that pattern will be emitted
    
    ( $value_0:expr, $value_1:expr, $value_2:expr ) => { //// passing multiple object syntax
        // ...
    };

    ( $($name:expr => $value:expr)* ) => { //// passing multiple key => value syntax 
        // ...

    };

}


#[macro_export]
macro_rules! log {
    ($arg:tt) => { //// passing single String message 
        $crate::env::log($arg.as_bytes()) //// log function only accepts utf8 bytes
    };
    ($($arg:tt)*) => { //// passing multiple String messages 
        $crate::env::log(format!($($arg)*).as_bytes()) //// log function only accepts utf8 bytes
    };
}


#[macro_export]
macro_rules! impl_engine_constructor {
    ($( $new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => { //// the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc 
        $(
            pub fn $new() -> Self{
                Self{
                    positions: [$( $pos ),*].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )* //// * means defining function for every new Pos
    };
}


// #[derive(Debug, Clone)]
// pub struct Shape{
//     typ: &'static str,
//     positions: HashSet<Pos>,
//     anchor: Pos,
// }


// #[derive(Debug, Clone, Copy)]
// pub struct Pos(pub i32, pub i32);



// impl Shape {
//     impl_engine_constructor! {
//       new_i "🟦": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
//       new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//       new_t "🟫": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
//       new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
//       new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
//       new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
//       new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//     }
// }