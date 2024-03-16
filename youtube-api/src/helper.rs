/// easy json path
macro_rules! jun {
    // access first field
    ($id:tt => $first:literal $($tail:tt)*) => {

       jun!(@push
            $id.get($first)
            $($tail)*
       )
    };

    // access field
    (@push $carry:expr => $field:literal $($tail:tt)*) => {

        jun!(@push
            $carry.and_then(|v| v.get($field))
            $($tail)*
        )
    };

    // move into array and flatten
    (@push $carry:expr => flat_map $($tail:tt)*) => {

        jun!(@push
            $carry
                .and_then(|v| v.as_array())
                .map(|v| v
                    .iter()
                    .flat_map(|v| jun!(v $($tail)*))
                    .flatten()
                    .collect::<Vec<_>>()
                )
        )
    };

    // move into array
    (@push $carry:expr => map $($tail:tt)*) => {

        jun!(@push
            $carry
                .and_then(|v| v.as_array())
                .map(|v| v
                    .iter()
                    .flat_map(|v| jun!(v $($tail)*))
                    .collect::<Vec<_>>()
                )
        )
    };

    // done
    (@push $carry:expr) => {

        $carry
    };

    // outer callback
    ($carry:expr => to $callback:expr) => {

        $callback($carry)
    };

    // inner callback
    (@push $carry:expr => to $callback:expr) => {

        $carry.and_then($callback)
    };

    (@push $carry:expr => to_string) => {

        $carry.and_then(|v| v.as_str())
        .map(|v| v.to_string())
    };
}

pub(crate) use jun;
