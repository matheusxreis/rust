/*
    Privacity Rules

    1. If a item is public, it can be accessed throught any its father modules
    2. If a item is private, it only can be accessed by its first father module and any father's child module.

*/
use crate::outermost as other_outermost;


mod outermost {

    pub fn middle_function(){}

    // this function just can be acessed by the outermost mod and inside mod
    fn middle_secret_function(){}
    
    mod inside {
        use crate::outermost;
        pub fn inner_function() {
            outermost::middle_secret_function()
        }
        fn secret_function() {}
    }

}

fn try_me(){
    outermost::middle_function(); // this dont give any error, bcs mod is declared here and middle_function is public
     // outermost::middle_secret_function(); // erro, function is private 
      // other_outermost::middle_secret_function(); // error continues yet
     // outermost::inside::inner_function() // erro, inside module is private
}