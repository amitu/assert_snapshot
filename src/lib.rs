extern crate failure;
extern crate itertools;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate sorted_json;

mod assert_snapshot;
mod destination;
mod function;
mod scrub;

pub use assert_snapshot::{assert_snapshot_, assert_snapshot_2};

#[macro_export]
macro_rules! assert_snapshot {
    ( $data:expr ) => {{
        assert_snapshot::assert_snapshot_2(file!(), line!(), "", $data).unwrap()
    }};
    ( $data:expr, $scrubs:expr ) => {{
        assert_snapshot::assert_snapshot_(file!(), line!(), "", $data, $scrubs).unwrap()
    }};
    ( $data:expr; $extra:expr   ) => {{
        assert_snapshot::assert_snapshot_2(file!(), line!(), $extra, $data).unwrap()
    }};
    ( $data:expr, $scrubs:expr; $extra:expr ) => {{
        assert_snapshot::assert_snapshot_(file!(), line!(), $extra, $data, $scrubs).unwrap()
    }};
}
