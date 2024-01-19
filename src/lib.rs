pub mod sila_domain;
pub mod binding;

pub mod server;
pub mod mapping;

pub mod sila {
    tonic::include_proto!("sila_codegen");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
