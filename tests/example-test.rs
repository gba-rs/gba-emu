#[cfg(test)]
mod tests {
    #[test]
    fn run_on_ci() {
        println!("Tests running on CI.");
        assert_eq!(2 + 2, 4);
    }
}