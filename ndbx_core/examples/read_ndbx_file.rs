extern crate ndbx_core;
extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
  const INDENT: &'static str = "    ";
  (0..size)
    .map(|_| INDENT)
    .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

// fn parse_ndbx(parser: &EventReader) -> Result<(), Err> {
//   for e in parser {
//     match e {
//       Ok(XmlEvent::StartElement {name, ..}) => {
//         if name.local_name != "ndbx" {
//           return Err("Document does not start with <ndbx> tag.");
//         }
//         parse_

//         }
//       }
//     }

//   }

// }

// fn parse_doc(parser: &EventReader) -> Result<(), Err> {
//   for e in parser {
//     match e {
//       Ok(XmlEvent::StartElement {name, ..}) => {
//         if name.local_name != "ndbx" {
//           return Err("Document does not start with <ndbx> tag.");
//         }
//         return parse_ndbx(parser);
//         }
//          Ok(XmlEvent::EndElement { name }) => {
//         if name.local_name != "ndbx" {
//           return Err("Document does not end with <ndbx> tag.");
//         }
//         return
//       }
//       }
//     }

//   }

// }

fn main() {
  let file = File::open("examples/corevector.ndbx").unwrap();
  let file = BufReader::new(file);
  let parser = EventReader::new(file);
  // parse_ndbx(e);
  let mut depth = 0;
  for e in parser {
    match e {
      Ok(XmlEvent::StartElement { name, .. }) => {
        if name.local_name == "ndbx" {
          println!("Start of ndbx doc");
        }
        println!("{}+{}", indent(depth), name);
        depth += 1;
      }
      Ok(XmlEvent::EndElement { name }) => {
        depth -= 1;
        println!("{}-{}", indent(depth), name);
      }
      Err(e) => {
        println!("Error: {}", e);
        break;
      }
      _ => {}
    }
  }
}
