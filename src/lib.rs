#[macro_use]
extern crate log;

#[macro_use]
extern crate enum_primitive;
extern crate num;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml;

mod enums;
pub mod cardxml;

#[cfg(test)]
mod tests {
    use cardxml;

    #[test]
    fn cardxml_load() {
      let cards = cardxml::load();
      assert!(cards.len() != 0);
    }
}
