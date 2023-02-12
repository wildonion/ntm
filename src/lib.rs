


use near_sdk::*;


// event!{

//     name: "list_owner",
//     log: [NewOwner, AddDeposit],

//     // event methods

//     fn add_owner(){

//     } 

//     fn add_deposit(){
        
//     }


// }

// emit!{
//     event_name
// }

#[macro_export]
macro_rules! contract {

    ($name:expr, $signer:expr, [$($fields:ident),*]; [$($method_type:expr => $method:ty),*]) => {

        {
            
        }

    }

}



contract!{

    "rev", //// name of the contract
    "wildonion.near", //// the contract owner
    [contract_owner, deposit_by_owner, contract_balance]; //// fields
    [ //// contract methods
        "init" => fn init_contract(){

        },
        "private" => fn get_all_deposits(){

        },
        "payable" => fn deposit(){
    
        },
        "external" => fn get_address_bytes(){

        }

    ]

}