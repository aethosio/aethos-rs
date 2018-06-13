extern crate nexus;

#[cfg(test)]
mod tests {
  #[test]
  fn implementation_is_not_visible() {
    use nexus::{ Container };
    let x: Container = Container::new();
  }
}
