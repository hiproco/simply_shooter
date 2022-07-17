pub mod enemy;
pub mod player;
pub mod projectile;
pub mod velocity;


#[cfg(test)]
mod tests {

    struct IO<F>(F);

    impl<O,F:Fn()->O> IO<F> {
        fn run(self) -> O {
            self.0()
        }
    }

    #[test]
    fn futures_io_monad() {
        let io = Some(());
        io.iter().map(|_| println!("da"));
    }
}