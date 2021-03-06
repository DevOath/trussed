
macro_rules! derive_key { ($($mechanism:ident),*) => {
    match request.mechanism {

        $(
            // #[cfg(feature = "ed255")]
            Mechanism::$mechanism => mechanisms::$mechanism::derive_key(self, request)
                .map(|reply| Reply::DeriveKey(reply)),
        )*

        _ => Err(Error::MechanismNotAvailable),

    }
}}
