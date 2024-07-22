mod my_mod{
    fn private_function(){
        println!("Called |>`my_mode::private_function()`");
    }

    pub fn function(){
        println!("Called |>`my_mod::function()`");
    }

    pub fn indirect_access(){
        println!("Called |>`my_mod::indirect_access()`");
        private_function();
    }

    pub mod nested {
        pub fn function(){
            println!("Called |>`my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("Called |>`my_mod::nested::private_function()`")
        }
        
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            println!("Called |>`my_mod::nested::public_funtion_in_my_mod()`");
            print!("----|>");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested(){
            println!("Called |>`my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod(){
            println!("Called |>`my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod(){
        println!("Called |>`my_mod::call_public_function_in_my_mod()`");
        print!("----|>");
        nested::public_function_in_my_mod();
        print!("----|>");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate(){
        println!("Called |>`my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function(){
            println!("Called |>`my_mod::private_nested::function()`");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("Called |>`my_mod::private_nested::restricted_function()`");
            }
    }

}
fn main() {
    my_mod::function();
    print!("----|>");
    my_mod::indirect_access();
    print!("----|>");
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();
}
