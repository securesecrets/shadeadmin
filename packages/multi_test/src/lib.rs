#[cfg(test)]
mod test;

use admin;
multi_derive::implement_multi!(AdminAuth, admin);
