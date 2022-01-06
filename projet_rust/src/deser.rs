use crate::style::Style;
use std::io::{self, Write};


use serde::{Serialize, Deserialize};
use create::deser;

use termion::{clear, raw::IntoRawMode};
use tui::{backend::TermionBackend, layout::*, widgets::*, *};
use {deser_hjson::*, serde::Deserialize, std::collections::HashMap};
use serde_json::from_str;

#[derive(Debug)]
struct Perso {
    nom: String,
    charisme: u64,
    force: u64,
    intellignece: u64,
}


#[derive(Deserialize, PartialEq, Debug)]
struct Current {
     description: String,
     choix_1:String,
      N_1:i32,
     choix_2: String,
      N_2:i32,
     choix_3: String,
      N_3:i32,

    missing: Option<f64>,
}


pub fn deser(txt:&str)-> Current {// doc hjson
    let hjson = txt ;
    let myhson = from_str(hjson).unwrap();
    return myhson;
}




/*
fn main() {
    // This Hjson document comes from https://hjson.github.io/
    //r# txt #
let docs = YamlLoader::load_from_str(histoire).unwrap();
let valeur_waml:i32=0;
//debut boucle

let doc = &docs[0][valeur_waml];
let mut out_str = String::new();
let mut emitter = YamlEmitter::new(&mut out_str);
emitter.dump(doc).unwrap();

    let hjson = out_str ;




    "
{
  // use #, // or /**/ comments,
  // omit quotes for keys
  // and use multiline strings

     description:
    '''
   	Ceci correspond à l'histoire.
    '''


     choix_1:
    '''
   	Ceci correspond au choix 1.
    '''
      N_1:1



     choix_2:
    '''
   	Ceci correspond au choix 2.
    '''
      N_2:2



     choix_3:
     '''
   	Ceci correspond au choix 3.
     '''
      N_3:3


}
";
    // we'll deserialize it into this struct:



     let expected = current {

     	description:"Ceci correspond à l'histoire.".to_owned(),

    	 choix_1:"Ceci correspond au choix 1.".to_owned(),
     	 N_1:1,

    	 choix_2:"Ceci correspond au choix 2.".to_owned(),
   	 N_2:2,

    	 choix_3:"Ceci correspond au choix 3.".to_owned(),
     	 N_3:3,

    };
    // Here's the deserialization and the equality check:

    let myhson = from_str(hjson).unwrap();
    if entre==1
    valeur_waml = myhson.N_1
    else if entre ==2
     valeur_waml= myhson.N_2
    else if entre ==3
     valeur_waml=myhson.N_3
    else
    assert_eq!(expected, myhson);
    println!("{:?}", myhson);
}*/
