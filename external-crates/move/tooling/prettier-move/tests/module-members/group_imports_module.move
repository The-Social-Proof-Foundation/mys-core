// options:
// printWidth: 40
// useModuleLabel: true
// autoGroupImports: module

module prettier::group_imports;

use a::b as c;
use a::b::e as f;
use a::{b::g as f, b::h as i};

use mys::balance::{Self, Balance};
use mys::coin::{Self, Coin};
use mys::dynamic_field as df;
use mys::dynamic_object_field as dof;
use mys::event;
use mys::mys::MYS;
use mys::transfer_policy::{Self, TransferPolicy, TransferRequest};

use std::{
    string::String,
    ascii::String as ASCII,
    vector as vec,
    option::{Self as opt, Option},
    type_name::get as type_name_get,
};

public fun do_something() {}
