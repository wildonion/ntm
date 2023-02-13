


use std::collections::HashMap;
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

    ($name:ident, $signer:expr, //// ident can be used to pass struct
     [$($fields:ident: $type:ty),*]; 
     [$($method_type:expr => [$($method:item),*]),* ]) 
     
     => {
            #[near_bindgen]
            #[derive(near_sdk::borsh::BorshDeserialize, near_sdk::borsh::BorshSerialize, near_sdk::PanicOnDefault)]
            pub struct $name{
                $($fields: $type),*
            }

            impl $name{

                // TODO - implement methods here 
                // ...
            }
    }

}



contract!{

    NftContract, //// name of the contract
    "wildonion.near", //// the contract owner
    /////////////////////
    //// contract fields
    /////////////////////
    [
        contract_owner: AccountId, 
        deposit_by_owner: HashMap<AccountId, near_sdk::json_types::U128>, 
        contract_balance: near_sdk::json_types::U128
    ]; //// fields
    /////////////////////
    //// contract methods
    /////////////////////
    [ 
        "init" => [ //// array of init methods
            pub fn init_contract(){
    
            }
        ],
        "private" => [ //// array of private methods
            pub fn get_all_deposits(){

            }
        ],
        "payable" => [ //// array of payable methods
            pub fn deposit(){
    
            }
        ],
        "external" => [ //// array of external methods
            fn get_address_bytes(){

            }
        ]
    ]

}