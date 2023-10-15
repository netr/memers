pub use pink_lock::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod pink_lock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allLpTokenLockedCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allLpTokenLockedCount",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allNormalTokenLockedCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allNormalTokenLockedCount",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cumulativeLockInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cumulativeLockInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("factory"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("editLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("editLock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lockId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newUnlockDate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("editLockDescription"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("editLockDescription",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lockId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("description"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCumulativeLpTokenLockInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCumulativeLpTokenLockInfo",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("start"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("end"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PinkLock02.CumulativeLockInfo[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCumulativeLpTokenLockInfoAt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCumulativeLpTokenLockInfoAt",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PinkLock02.CumulativeLockInfo",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCumulativeNormalTokenLockInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCumulativeNormalTokenLockInfo",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("start"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("end"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PinkLock02.CumulativeLockInfo[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCumulativeNormalTokenLockInfoAt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getCumulativeNormalTokenLockInfoAt",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct PinkLock02.CumulativeLockInfo",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLockAt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLockAt"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::String,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLockById"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLockById"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("lockId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::String,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLocksForToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLocksForToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("start"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("end"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalLockCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalLockCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isLpToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("unlockDate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("description"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lpLockCountForUser"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpLockCountForUser"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lpLockForUserAtIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpLockForUserAtIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::String,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lpLocksForUser"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpLocksForUser"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multipleVestingLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("multipleVestingLock",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owners"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isLpToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tgeDate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tgeBps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cycle"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cycleBps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("description"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("normalLockCountForUser"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("normalLockCountForUser",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("normalLockForUserAtIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("normalLockForUserAtIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::String,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("normalLocksForUser"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("normalLocksForUser"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PinkLock02.Lock[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceLockOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceLockOwnership",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("lockId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalLockCountForToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalLockCountForToken",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalLockCountForUser"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalLockCountForUser",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalTokenLockedCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalTokenLockedCount",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferLockOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferLockOwnership",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lockId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unlock"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("lockId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("vestingLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("vestingLock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isLpToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tgeDate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tgeBps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cycle"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cycleBps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("description"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawableTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawableTokens"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("lockId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LockAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("unlockDate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockDescriptionChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockDescriptionChanged",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("lockId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockOwnerChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockOwnerChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("lockId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockRemoved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockRemoved"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("unlockedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newUnlockDate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockVested"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockVested"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("remaining"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PINKLOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct PinkLock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PinkLock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PinkLock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PinkLock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PinkLock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PinkLock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PinkLock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PINKLOCK_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `allLpTokenLockedCount` (0xb982922e) function
        pub fn all_lp_token_locked_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 130, 146, 46], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allNormalTokenLockedCount` (0x475831c8) function
        pub fn all_normal_token_locked_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 88, 49, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeLockInfo` (0xe1444fd6) function
        pub fn cumulative_lock_info(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([225, 68, 79, 214], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `editLock` (0xb3b9aa48) function
        pub fn edit_lock(
            &self,
            lock_id: ::ethers::core::types::U256,
            new_amount: ::ethers::core::types::U256,
            new_unlock_date: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 185, 170, 72], (lock_id, new_amount, new_unlock_date))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `editLockDescription` (0xd3cac885) function
        pub fn edit_lock_description(
            &self,
            lock_id: ::ethers::core::types::U256,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 202, 200, 133], (lock_id, description))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCumulativeLpTokenLockInfo` (0xaec640c6) function
        pub fn get_cumulative_lp_token_lock_info(
            &self,
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CumulativeLockInfo>>
        {
            self.0
                .method_hash([174, 198, 64, 198], (start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCumulativeLpTokenLockInfoAt` (0xa20b8c18) function
        pub fn get_cumulative_lp_token_lock_info_at(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, CumulativeLockInfo> {
            self.0
                .method_hash([162, 11, 140, 24], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCumulativeNormalTokenLockInfo` (0x76c12822) function
        pub fn get_cumulative_normal_token_lock_info(
            &self,
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CumulativeLockInfo>>
        {
            self.0
                .method_hash([118, 193, 40, 34], (start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCumulativeNormalTokenLockInfoAt` (0x7e6706d3) function
        pub fn get_cumulative_normal_token_lock_info_at(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, CumulativeLockInfo> {
            self.0
                .method_hash([126, 103, 6, 211], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLockAt` (0x0d4f581a) function
        pub fn get_lock_at(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Lock> {
            self.0
                .method_hash([13, 79, 88, 26], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLockById` (0x08f12470) function
        pub fn get_lock_by_id(
            &self,
            lock_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Lock> {
            self.0
                .method_hash([8, 241, 36, 112], lock_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLocksForToken` (0x332f26d7) function
        pub fn get_locks_for_token(
            &self,
            token: ::ethers::core::types::Address,
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Lock>> {
            self.0
                .method_hash([51, 47, 38, 215], (token, start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalLockCount` (0xfd981c66) function
        pub fn get_total_lock_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 152, 28, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lock` (0x07279357) function
        pub fn lock(
            &self,
            owner: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            is_lp_token: bool,
            amount: ::ethers::core::types::U256,
            unlock_date: ::ethers::core::types::U256,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [7, 39, 147, 87],
                    (owner, token, is_lp_token, amount, unlock_date, description),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpLockCountForUser` (0x07873ef1) function
        pub fn lp_lock_count_for_user(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 135, 62, 241], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpLockForUserAtIndex` (0xeeacf786) function
        pub fn lp_lock_for_user_at_index(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Lock> {
            self.0
                .method_hash([238, 172, 247, 134], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpLocksForUser` (0xaef0e540) function
        pub fn lp_locks_for_user(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Lock>> {
            self.0
                .method_hash([174, 240, 229, 64], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multipleVestingLock` (0xe0da83ce) function
        pub fn multiple_vesting_lock(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            token: ::ethers::core::types::Address,
            is_lp_token: bool,
            tge_date: ::ethers::core::types::U256,
            tge_bps: ::ethers::core::types::U256,
            cycle: ::ethers::core::types::U256,
            cycle_bps: ::ethers::core::types::U256,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [224, 218, 131, 206],
                    (
                        owners,
                        amounts,
                        token,
                        is_lp_token,
                        tge_date,
                        tge_bps,
                        cycle,
                        cycle_bps,
                        description,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `normalLockCountForUser` (0xeb80bdae) function
        pub fn normal_lock_count_for_user(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([235, 128, 189, 174], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `normalLockForUserAtIndex` (0x618df7a3) function
        pub fn normal_lock_for_user_at_index(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Lock> {
            self.0
                .method_hash([97, 141, 247, 163], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `normalLocksForUser` (0xda1d8cff) function
        pub fn normal_locks_for_user(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Lock>> {
            self.0
                .method_hash([218, 29, 140, 255], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceLockOwnership` (0xa57e3141) function
        pub fn renounce_lock_ownership(
            &self,
            lock_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 126, 49, 65], lock_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLockCountForToken` (0xe3676f88) function
        pub fn total_lock_count_for_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 103, 111, 136], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLockCountForUser` (0xcd83eadc) function
        pub fn total_lock_count_for_user(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 131, 234, 220], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalTokenLockedCount` (0x1982242c) function
        pub fn total_token_locked_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 130, 36, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferLockOwnership` (0x5a04fb69) function
        pub fn transfer_lock_ownership(
            &self,
            lock_id: ::ethers::core::types::U256,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 4, 251, 105], (lock_id, new_owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unlock` (0x6198e339) function
        pub fn unlock(
            &self,
            lock_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 152, 227, 57], lock_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vestingLock` (0xcb645e32) function
        pub fn vesting_lock(
            &self,
            owner: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            is_lp_token: bool,
            amount: ::ethers::core::types::U256,
            tge_date: ::ethers::core::types::U256,
            tge_bps: ::ethers::core::types::U256,
            cycle: ::ethers::core::types::U256,
            cycle_bps: ::ethers::core::types::U256,
            description: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [203, 100, 94, 50],
                    (
                        owner,
                        token,
                        is_lp_token,
                        amount,
                        tge_date,
                        tge_bps,
                        cycle,
                        cycle_bps,
                        description,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawableTokens` (0x6dbdeab3) function
        pub fn withdrawable_tokens(
            &self,
            lock_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 189, 234, 179], lock_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LockAdded` event
        pub fn lock_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockAddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `LockDescriptionChanged` event
        pub fn lock_description_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockDescriptionChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LockOwnerChanged` event
        pub fn lock_owner_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockOwnerChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LockRemoved` event
        pub fn lock_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockRemovedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LockUpdated` event
        pub fn lock_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LockVested` event
        pub fn lock_vested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockVestedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PinkLockEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PinkLock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LockAdded",
        abi = "LockAdded(uint256,address,address,uint256,uint256)"
    )]
    pub struct LockAddedFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub unlock_date: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LockDescriptionChanged",
        abi = "LockDescriptionChanged(uint256)"
    )]
    pub struct LockDescriptionChangedFilter {
        pub lock_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LockOwnerChanged",
        abi = "LockOwnerChanged(uint256,address,address)"
    )]
    pub struct LockOwnerChangedFilter {
        pub lock_id: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LockRemoved",
        abi = "LockRemoved(uint256,address,address,uint256,uint256)"
    )]
    pub struct LockRemovedFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub unlocked_at: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LockUpdated",
        abi = "LockUpdated(uint256,address,address,uint256,uint256)"
    )]
    pub struct LockUpdatedFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub new_amount: ::ethers::core::types::U256,
        pub new_unlock_date: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LockVested",
        abi = "LockVested(uint256,address,address,uint256,uint256,uint256)"
    )]
    pub struct LockVestedFilter {
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub remaining: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PinkLockEvents {
        LockAddedFilter(LockAddedFilter),
        LockDescriptionChangedFilter(LockDescriptionChangedFilter),
        LockOwnerChangedFilter(LockOwnerChangedFilter),
        LockRemovedFilter(LockRemovedFilter),
        LockUpdatedFilter(LockUpdatedFilter),
        LockVestedFilter(LockVestedFilter),
    }
    impl ::ethers::contract::EthLogDecode for PinkLockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LockAddedFilter::decode_log(log) {
                return Ok(PinkLockEvents::LockAddedFilter(decoded));
            }
            if let Ok(decoded) = LockDescriptionChangedFilter::decode_log(log) {
                return Ok(PinkLockEvents::LockDescriptionChangedFilter(decoded));
            }
            if let Ok(decoded) = LockOwnerChangedFilter::decode_log(log) {
                return Ok(PinkLockEvents::LockOwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = LockRemovedFilter::decode_log(log) {
                return Ok(PinkLockEvents::LockRemovedFilter(decoded));
            }
            if let Ok(decoded) = LockUpdatedFilter::decode_log(log) {
                return Ok(PinkLockEvents::LockUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LockVestedFilter::decode_log(log) {
                return Ok(PinkLockEvents::LockVestedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PinkLockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LockAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockDescriptionChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockOwnerChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockRemovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockVestedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LockAddedFilter> for PinkLockEvents {
        fn from(value: LockAddedFilter) -> Self {
            Self::LockAddedFilter(value)
        }
    }
    impl ::core::convert::From<LockDescriptionChangedFilter> for PinkLockEvents {
        fn from(value: LockDescriptionChangedFilter) -> Self {
            Self::LockDescriptionChangedFilter(value)
        }
    }
    impl ::core::convert::From<LockOwnerChangedFilter> for PinkLockEvents {
        fn from(value: LockOwnerChangedFilter) -> Self {
            Self::LockOwnerChangedFilter(value)
        }
    }
    impl ::core::convert::From<LockRemovedFilter> for PinkLockEvents {
        fn from(value: LockRemovedFilter) -> Self {
            Self::LockRemovedFilter(value)
        }
    }
    impl ::core::convert::From<LockUpdatedFilter> for PinkLockEvents {
        fn from(value: LockUpdatedFilter) -> Self {
            Self::LockUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LockVestedFilter> for PinkLockEvents {
        fn from(value: LockVestedFilter) -> Self {
            Self::LockVestedFilter(value)
        }
    }
    ///Container type for all input parameters for the `allLpTokenLockedCount` function with signature `allLpTokenLockedCount()` and selector `0xb982922e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "allLpTokenLockedCount", abi = "allLpTokenLockedCount()")]
    pub struct AllLpTokenLockedCountCall;
    ///Container type for all input parameters for the `allNormalTokenLockedCount` function with signature `allNormalTokenLockedCount()` and selector `0x475831c8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "allNormalTokenLockedCount",
        abi = "allNormalTokenLockedCount()"
    )]
    pub struct AllNormalTokenLockedCountCall;
    ///Container type for all input parameters for the `cumulativeLockInfo` function with signature `cumulativeLockInfo(address)` and selector `0xe1444fd6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "cumulativeLockInfo", abi = "cumulativeLockInfo(address)")]
    pub struct CumulativeLockInfoCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `editLock` function with signature `editLock(uint256,uint256,uint256)` and selector `0xb3b9aa48`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "editLock", abi = "editLock(uint256,uint256,uint256)")]
    pub struct EditLockCall {
        pub lock_id: ::ethers::core::types::U256,
        pub new_amount: ::ethers::core::types::U256,
        pub new_unlock_date: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `editLockDescription` function with signature `editLockDescription(uint256,string)` and selector `0xd3cac885`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "editLockDescription",
        abi = "editLockDescription(uint256,string)"
    )]
    pub struct EditLockDescriptionCall {
        pub lock_id: ::ethers::core::types::U256,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `getCumulativeLpTokenLockInfo` function with signature `getCumulativeLpTokenLockInfo(uint256,uint256)` and selector `0xaec640c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getCumulativeLpTokenLockInfo",
        abi = "getCumulativeLpTokenLockInfo(uint256,uint256)"
    )]
    pub struct GetCumulativeLpTokenLockInfoCall {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getCumulativeLpTokenLockInfoAt` function with signature `getCumulativeLpTokenLockInfoAt(uint256)` and selector `0xa20b8c18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getCumulativeLpTokenLockInfoAt",
        abi = "getCumulativeLpTokenLockInfoAt(uint256)"
    )]
    pub struct GetCumulativeLpTokenLockInfoAtCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getCumulativeNormalTokenLockInfo` function with signature `getCumulativeNormalTokenLockInfo(uint256,uint256)` and selector `0x76c12822`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getCumulativeNormalTokenLockInfo",
        abi = "getCumulativeNormalTokenLockInfo(uint256,uint256)"
    )]
    pub struct GetCumulativeNormalTokenLockInfoCall {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getCumulativeNormalTokenLockInfoAt` function with signature `getCumulativeNormalTokenLockInfoAt(uint256)` and selector `0x7e6706d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getCumulativeNormalTokenLockInfoAt",
        abi = "getCumulativeNormalTokenLockInfoAt(uint256)"
    )]
    pub struct GetCumulativeNormalTokenLockInfoAtCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLockAt` function with signature `getLockAt(uint256)` and selector `0x0d4f581a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLockAt", abi = "getLockAt(uint256)")]
    pub struct GetLockAtCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLockById` function with signature `getLockById(uint256)` and selector `0x08f12470`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLockById", abi = "getLockById(uint256)")]
    pub struct GetLockByIdCall {
        pub lock_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLocksForToken` function with signature `getLocksForToken(address,uint256,uint256)` and selector `0x332f26d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getLocksForToken",
        abi = "getLocksForToken(address,uint256,uint256)"
    )]
    pub struct GetLocksForTokenCall {
        pub token: ::ethers::core::types::Address,
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalLockCount` function with signature `getTotalLockCount()` and selector `0xfd981c66`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTotalLockCount", abi = "getTotalLockCount()")]
    pub struct GetTotalLockCountCall;
    ///Container type for all input parameters for the `lock` function with signature `lock(address,address,bool,uint256,uint256,string)` and selector `0x07279357`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "lock",
        abi = "lock(address,address,bool,uint256,uint256,string)"
    )]
    pub struct LockCall {
        pub owner: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub is_lp_token: bool,
        pub amount: ::ethers::core::types::U256,
        pub unlock_date: ::ethers::core::types::U256,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `lpLockCountForUser` function with signature `lpLockCountForUser(address)` and selector `0x07873ef1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lpLockCountForUser", abi = "lpLockCountForUser(address)")]
    pub struct LpLockCountForUserCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lpLockForUserAtIndex` function with signature `lpLockForUserAtIndex(address,uint256)` and selector `0xeeacf786`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "lpLockForUserAtIndex",
        abi = "lpLockForUserAtIndex(address,uint256)"
    )]
    pub struct LpLockForUserAtIndexCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lpLocksForUser` function with signature `lpLocksForUser(address)` and selector `0xaef0e540`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lpLocksForUser", abi = "lpLocksForUser(address)")]
    pub struct LpLocksForUserCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `multipleVestingLock` function with signature `multipleVestingLock(address[],uint256[],address,bool,uint256,uint256,uint256,uint256,string)` and selector `0xe0da83ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "multipleVestingLock",
        abi = "multipleVestingLock(address[],uint256[],address,bool,uint256,uint256,uint256,uint256,string)"
    )]
    pub struct MultipleVestingLockCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub token: ::ethers::core::types::Address,
        pub is_lp_token: bool,
        pub tge_date: ::ethers::core::types::U256,
        pub tge_bps: ::ethers::core::types::U256,
        pub cycle: ::ethers::core::types::U256,
        pub cycle_bps: ::ethers::core::types::U256,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `normalLockCountForUser` function with signature `normalLockCountForUser(address)` and selector `0xeb80bdae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "normalLockCountForUser",
        abi = "normalLockCountForUser(address)"
    )]
    pub struct NormalLockCountForUserCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `normalLockForUserAtIndex` function with signature `normalLockForUserAtIndex(address,uint256)` and selector `0x618df7a3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "normalLockForUserAtIndex",
        abi = "normalLockForUserAtIndex(address,uint256)"
    )]
    pub struct NormalLockForUserAtIndexCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `normalLocksForUser` function with signature `normalLocksForUser(address)` and selector `0xda1d8cff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "normalLocksForUser", abi = "normalLocksForUser(address)")]
    pub struct NormalLocksForUserCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceLockOwnership` function with signature `renounceLockOwnership(uint256)` and selector `0xa57e3141`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceLockOwnership", abi = "renounceLockOwnership(uint256)")]
    pub struct RenounceLockOwnershipCall {
        pub lock_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalLockCountForToken` function with signature `totalLockCountForToken(address)` and selector `0xe3676f88`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "totalLockCountForToken",
        abi = "totalLockCountForToken(address)"
    )]
    pub struct TotalLockCountForTokenCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `totalLockCountForUser` function with signature `totalLockCountForUser(address)` and selector `0xcd83eadc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalLockCountForUser", abi = "totalLockCountForUser(address)")]
    pub struct TotalLockCountForUserCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `totalTokenLockedCount` function with signature `totalTokenLockedCount()` and selector `0x1982242c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalTokenLockedCount", abi = "totalTokenLockedCount()")]
    pub struct TotalTokenLockedCountCall;
    ///Container type for all input parameters for the `transferLockOwnership` function with signature `transferLockOwnership(uint256,address)` and selector `0x5a04fb69`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "transferLockOwnership",
        abi = "transferLockOwnership(uint256,address)"
    )]
    pub struct TransferLockOwnershipCall {
        pub lock_id: ::ethers::core::types::U256,
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unlock` function with signature `unlock(uint256)` and selector `0x6198e339`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "unlock", abi = "unlock(uint256)")]
    pub struct UnlockCall {
        pub lock_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `vestingLock` function with signature `vestingLock(address,address,bool,uint256,uint256,uint256,uint256,uint256,string)` and selector `0xcb645e32`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "vestingLock",
        abi = "vestingLock(address,address,bool,uint256,uint256,uint256,uint256,uint256,string)"
    )]
    pub struct VestingLockCall {
        pub owner: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub is_lp_token: bool,
        pub amount: ::ethers::core::types::U256,
        pub tge_date: ::ethers::core::types::U256,
        pub tge_bps: ::ethers::core::types::U256,
        pub cycle: ::ethers::core::types::U256,
        pub cycle_bps: ::ethers::core::types::U256,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `withdrawableTokens` function with signature `withdrawableTokens(uint256)` and selector `0x6dbdeab3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdrawableTokens", abi = "withdrawableTokens(uint256)")]
    pub struct WithdrawableTokensCall {
        pub lock_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PinkLockCalls {
        AllLpTokenLockedCount(AllLpTokenLockedCountCall),
        AllNormalTokenLockedCount(AllNormalTokenLockedCountCall),
        CumulativeLockInfo(CumulativeLockInfoCall),
        EditLock(EditLockCall),
        EditLockDescription(EditLockDescriptionCall),
        GetCumulativeLpTokenLockInfo(GetCumulativeLpTokenLockInfoCall),
        GetCumulativeLpTokenLockInfoAt(GetCumulativeLpTokenLockInfoAtCall),
        GetCumulativeNormalTokenLockInfo(GetCumulativeNormalTokenLockInfoCall),
        GetCumulativeNormalTokenLockInfoAt(GetCumulativeNormalTokenLockInfoAtCall),
        GetLockAt(GetLockAtCall),
        GetLockById(GetLockByIdCall),
        GetLocksForToken(GetLocksForTokenCall),
        GetTotalLockCount(GetTotalLockCountCall),
        Lock(LockCall),
        LpLockCountForUser(LpLockCountForUserCall),
        LpLockForUserAtIndex(LpLockForUserAtIndexCall),
        LpLocksForUser(LpLocksForUserCall),
        MultipleVestingLock(MultipleVestingLockCall),
        NormalLockCountForUser(NormalLockCountForUserCall),
        NormalLockForUserAtIndex(NormalLockForUserAtIndexCall),
        NormalLocksForUser(NormalLocksForUserCall),
        RenounceLockOwnership(RenounceLockOwnershipCall),
        TotalLockCountForToken(TotalLockCountForTokenCall),
        TotalLockCountForUser(TotalLockCountForUserCall),
        TotalTokenLockedCount(TotalTokenLockedCountCall),
        TransferLockOwnership(TransferLockOwnershipCall),
        Unlock(UnlockCall),
        VestingLock(VestingLockCall),
        WithdrawableTokens(WithdrawableTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for PinkLockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AllLpTokenLockedCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllLpTokenLockedCount(decoded));
            }
            if let Ok(decoded) =
                <AllNormalTokenLockedCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllNormalTokenLockedCount(decoded));
            }
            if let Ok(decoded) =
                <CumulativeLockInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeLockInfo(decoded));
            }
            if let Ok(decoded) = <EditLockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EditLock(decoded));
            }
            if let Ok(decoded) =
                <EditLockDescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EditLockDescription(decoded));
            }
            if let Ok(decoded) =
                <GetCumulativeLpTokenLockInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCumulativeLpTokenLockInfo(decoded));
            }
            if let Ok(decoded) =
                <GetCumulativeLpTokenLockInfoAtCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCumulativeLpTokenLockInfoAt(decoded));
            }
            if let Ok(decoded) =
                <GetCumulativeNormalTokenLockInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetCumulativeNormalTokenLockInfo(decoded));
            }
            if let Ok(decoded) =
                <GetCumulativeNormalTokenLockInfoAtCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetCumulativeNormalTokenLockInfoAt(decoded));
            }
            if let Ok(decoded) = <GetLockAtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLockAt(decoded));
            }
            if let Ok(decoded) = <GetLockByIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLockById(decoded));
            }
            if let Ok(decoded) =
                <GetLocksForTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLocksForToken(decoded));
            }
            if let Ok(decoded) =
                <GetTotalLockCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTotalLockCount(decoded));
            }
            if let Ok(decoded) = <LockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Lock(decoded));
            }
            if let Ok(decoded) =
                <LpLockCountForUserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LpLockCountForUser(decoded));
            }
            if let Ok(decoded) =
                <LpLockForUserAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LpLockForUserAtIndex(decoded));
            }
            if let Ok(decoded) =
                <LpLocksForUserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LpLocksForUser(decoded));
            }
            if let Ok(decoded) =
                <MultipleVestingLockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MultipleVestingLock(decoded));
            }
            if let Ok(decoded) =
                <NormalLockCountForUserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalLockCountForUser(decoded));
            }
            if let Ok(decoded) =
                <NormalLockForUserAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalLockForUserAtIndex(decoded));
            }
            if let Ok(decoded) =
                <NormalLocksForUserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalLocksForUser(decoded));
            }
            if let Ok(decoded) =
                <RenounceLockOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceLockOwnership(decoded));
            }
            if let Ok(decoded) =
                <TotalLockCountForTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalLockCountForToken(decoded));
            }
            if let Ok(decoded) =
                <TotalLockCountForUserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalLockCountForUser(decoded));
            }
            if let Ok(decoded) =
                <TotalTokenLockedCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalTokenLockedCount(decoded));
            }
            if let Ok(decoded) =
                <TransferLockOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferLockOwnership(decoded));
            }
            if let Ok(decoded) = <UnlockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unlock(decoded));
            }
            if let Ok(decoded) = <VestingLockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VestingLock(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawableTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PinkLockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllLpTokenLockedCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllNormalTokenLockedCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CumulativeLockInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EditLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EditLockDescription(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCumulativeLpTokenLockInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCumulativeLpTokenLockInfoAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCumulativeNormalTokenLockInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCumulativeNormalTokenLockInfoAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLockAt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLockById(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLocksForToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTotalLockCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Lock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LpLockCountForUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LpLockForUserAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LpLocksForUser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MultipleVestingLock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalLockCountForUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalLockForUserAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalLocksForUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceLockOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalLockCountForToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalLockCountForUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalTokenLockedCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferLockOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unlock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VestingLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawableTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PinkLockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllLpTokenLockedCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllNormalTokenLockedCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeLockInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditLockDescription(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCumulativeLpTokenLockInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCumulativeLpTokenLockInfoAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCumulativeNormalTokenLockInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCumulativeNormalTokenLockInfoAt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLockAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLockById(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLocksForToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalLockCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lock(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpLockCountForUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpLockForUserAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpLocksForUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultipleVestingLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::NormalLockCountForUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::NormalLockForUserAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::NormalLocksForUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceLockOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLockCountForToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLockCountForUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalTokenLockedCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferLockOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::VestingLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllLpTokenLockedCountCall> for PinkLockCalls {
        fn from(value: AllLpTokenLockedCountCall) -> Self {
            Self::AllLpTokenLockedCount(value)
        }
    }
    impl ::core::convert::From<AllNormalTokenLockedCountCall> for PinkLockCalls {
        fn from(value: AllNormalTokenLockedCountCall) -> Self {
            Self::AllNormalTokenLockedCount(value)
        }
    }
    impl ::core::convert::From<CumulativeLockInfoCall> for PinkLockCalls {
        fn from(value: CumulativeLockInfoCall) -> Self {
            Self::CumulativeLockInfo(value)
        }
    }
    impl ::core::convert::From<EditLockCall> for PinkLockCalls {
        fn from(value: EditLockCall) -> Self {
            Self::EditLock(value)
        }
    }
    impl ::core::convert::From<EditLockDescriptionCall> for PinkLockCalls {
        fn from(value: EditLockDescriptionCall) -> Self {
            Self::EditLockDescription(value)
        }
    }
    impl ::core::convert::From<GetCumulativeLpTokenLockInfoCall> for PinkLockCalls {
        fn from(value: GetCumulativeLpTokenLockInfoCall) -> Self {
            Self::GetCumulativeLpTokenLockInfo(value)
        }
    }
    impl ::core::convert::From<GetCumulativeLpTokenLockInfoAtCall> for PinkLockCalls {
        fn from(value: GetCumulativeLpTokenLockInfoAtCall) -> Self {
            Self::GetCumulativeLpTokenLockInfoAt(value)
        }
    }
    impl ::core::convert::From<GetCumulativeNormalTokenLockInfoCall> for PinkLockCalls {
        fn from(value: GetCumulativeNormalTokenLockInfoCall) -> Self {
            Self::GetCumulativeNormalTokenLockInfo(value)
        }
    }
    impl ::core::convert::From<GetCumulativeNormalTokenLockInfoAtCall> for PinkLockCalls {
        fn from(value: GetCumulativeNormalTokenLockInfoAtCall) -> Self {
            Self::GetCumulativeNormalTokenLockInfoAt(value)
        }
    }
    impl ::core::convert::From<GetLockAtCall> for PinkLockCalls {
        fn from(value: GetLockAtCall) -> Self {
            Self::GetLockAt(value)
        }
    }
    impl ::core::convert::From<GetLockByIdCall> for PinkLockCalls {
        fn from(value: GetLockByIdCall) -> Self {
            Self::GetLockById(value)
        }
    }
    impl ::core::convert::From<GetLocksForTokenCall> for PinkLockCalls {
        fn from(value: GetLocksForTokenCall) -> Self {
            Self::GetLocksForToken(value)
        }
    }
    impl ::core::convert::From<GetTotalLockCountCall> for PinkLockCalls {
        fn from(value: GetTotalLockCountCall) -> Self {
            Self::GetTotalLockCount(value)
        }
    }
    impl ::core::convert::From<LockCall> for PinkLockCalls {
        fn from(value: LockCall) -> Self {
            Self::Lock(value)
        }
    }
    impl ::core::convert::From<LpLockCountForUserCall> for PinkLockCalls {
        fn from(value: LpLockCountForUserCall) -> Self {
            Self::LpLockCountForUser(value)
        }
    }
    impl ::core::convert::From<LpLockForUserAtIndexCall> for PinkLockCalls {
        fn from(value: LpLockForUserAtIndexCall) -> Self {
            Self::LpLockForUserAtIndex(value)
        }
    }
    impl ::core::convert::From<LpLocksForUserCall> for PinkLockCalls {
        fn from(value: LpLocksForUserCall) -> Self {
            Self::LpLocksForUser(value)
        }
    }
    impl ::core::convert::From<MultipleVestingLockCall> for PinkLockCalls {
        fn from(value: MultipleVestingLockCall) -> Self {
            Self::MultipleVestingLock(value)
        }
    }
    impl ::core::convert::From<NormalLockCountForUserCall> for PinkLockCalls {
        fn from(value: NormalLockCountForUserCall) -> Self {
            Self::NormalLockCountForUser(value)
        }
    }
    impl ::core::convert::From<NormalLockForUserAtIndexCall> for PinkLockCalls {
        fn from(value: NormalLockForUserAtIndexCall) -> Self {
            Self::NormalLockForUserAtIndex(value)
        }
    }
    impl ::core::convert::From<NormalLocksForUserCall> for PinkLockCalls {
        fn from(value: NormalLocksForUserCall) -> Self {
            Self::NormalLocksForUser(value)
        }
    }
    impl ::core::convert::From<RenounceLockOwnershipCall> for PinkLockCalls {
        fn from(value: RenounceLockOwnershipCall) -> Self {
            Self::RenounceLockOwnership(value)
        }
    }
    impl ::core::convert::From<TotalLockCountForTokenCall> for PinkLockCalls {
        fn from(value: TotalLockCountForTokenCall) -> Self {
            Self::TotalLockCountForToken(value)
        }
    }
    impl ::core::convert::From<TotalLockCountForUserCall> for PinkLockCalls {
        fn from(value: TotalLockCountForUserCall) -> Self {
            Self::TotalLockCountForUser(value)
        }
    }
    impl ::core::convert::From<TotalTokenLockedCountCall> for PinkLockCalls {
        fn from(value: TotalTokenLockedCountCall) -> Self {
            Self::TotalTokenLockedCount(value)
        }
    }
    impl ::core::convert::From<TransferLockOwnershipCall> for PinkLockCalls {
        fn from(value: TransferLockOwnershipCall) -> Self {
            Self::TransferLockOwnership(value)
        }
    }
    impl ::core::convert::From<UnlockCall> for PinkLockCalls {
        fn from(value: UnlockCall) -> Self {
            Self::Unlock(value)
        }
    }
    impl ::core::convert::From<VestingLockCall> for PinkLockCalls {
        fn from(value: VestingLockCall) -> Self {
            Self::VestingLock(value)
        }
    }
    impl ::core::convert::From<WithdrawableTokensCall> for PinkLockCalls {
        fn from(value: WithdrawableTokensCall) -> Self {
            Self::WithdrawableTokens(value)
        }
    }
    ///Container type for all return fields from the `allLpTokenLockedCount` function with signature `allLpTokenLockedCount()` and selector `0xb982922e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AllLpTokenLockedCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allNormalTokenLockedCount` function with signature `allNormalTokenLockedCount()` and selector `0x475831c8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AllNormalTokenLockedCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `cumulativeLockInfo` function with signature `cumulativeLockInfo(address)` and selector `0xe1444fd6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CumulativeLockInfoReturn {
        pub token: ::ethers::core::types::Address,
        pub factory: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getCumulativeLpTokenLockInfo` function with signature `getCumulativeLpTokenLockInfo(uint256,uint256)` and selector `0xaec640c6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCumulativeLpTokenLockInfoReturn(pub ::std::vec::Vec<CumulativeLockInfo>);
    ///Container type for all return fields from the `getCumulativeLpTokenLockInfoAt` function with signature `getCumulativeLpTokenLockInfoAt(uint256)` and selector `0xa20b8c18`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCumulativeLpTokenLockInfoAtReturn(pub CumulativeLockInfo);
    ///Container type for all return fields from the `getCumulativeNormalTokenLockInfo` function with signature `getCumulativeNormalTokenLockInfo(uint256,uint256)` and selector `0x76c12822`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCumulativeNormalTokenLockInfoReturn(pub ::std::vec::Vec<CumulativeLockInfo>);
    ///Container type for all return fields from the `getCumulativeNormalTokenLockInfoAt` function with signature `getCumulativeNormalTokenLockInfoAt(uint256)` and selector `0x7e6706d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCumulativeNormalTokenLockInfoAtReturn(pub CumulativeLockInfo);
    ///Container type for all return fields from the `getLockAt` function with signature `getLockAt(uint256)` and selector `0x0d4f581a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLockAtReturn(pub Lock);
    ///Container type for all return fields from the `getLockById` function with signature `getLockById(uint256)` and selector `0x08f12470`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLockByIdReturn(pub Lock);
    ///Container type for all return fields from the `getLocksForToken` function with signature `getLocksForToken(address,uint256,uint256)` and selector `0x332f26d7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLocksForTokenReturn(pub ::std::vec::Vec<Lock>);
    ///Container type for all return fields from the `getTotalLockCount` function with signature `getTotalLockCount()` and selector `0xfd981c66`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTotalLockCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lock` function with signature `lock(address,address,bool,uint256,uint256,string)` and selector `0x07279357`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LockReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lpLockCountForUser` function with signature `lpLockCountForUser(address)` and selector `0x07873ef1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LpLockCountForUserReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lpLockForUserAtIndex` function with signature `lpLockForUserAtIndex(address,uint256)` and selector `0xeeacf786`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LpLockForUserAtIndexReturn(pub Lock);
    ///Container type for all return fields from the `lpLocksForUser` function with signature `lpLocksForUser(address)` and selector `0xaef0e540`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LpLocksForUserReturn(pub ::std::vec::Vec<Lock>);
    ///Container type for all return fields from the `multipleVestingLock` function with signature `multipleVestingLock(address[],uint256[],address,bool,uint256,uint256,uint256,uint256,string)` and selector `0xe0da83ce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MultipleVestingLockReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `normalLockCountForUser` function with signature `normalLockCountForUser(address)` and selector `0xeb80bdae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NormalLockCountForUserReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `normalLockForUserAtIndex` function with signature `normalLockForUserAtIndex(address,uint256)` and selector `0x618df7a3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NormalLockForUserAtIndexReturn(pub Lock);
    ///Container type for all return fields from the `normalLocksForUser` function with signature `normalLocksForUser(address)` and selector `0xda1d8cff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NormalLocksForUserReturn(pub ::std::vec::Vec<Lock>);
    ///Container type for all return fields from the `totalLockCountForToken` function with signature `totalLockCountForToken(address)` and selector `0xe3676f88`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalLockCountForTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalLockCountForUser` function with signature `totalLockCountForUser(address)` and selector `0xcd83eadc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalLockCountForUserReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalTokenLockedCount` function with signature `totalTokenLockedCount()` and selector `0x1982242c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalTokenLockedCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `vestingLock` function with signature `vestingLock(address,address,bool,uint256,uint256,uint256,uint256,uint256,string)` and selector `0xcb645e32`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VestingLockReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `withdrawableTokens` function with signature `withdrawableTokens(uint256)` and selector `0x6dbdeab3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawableTokensReturn(pub ::ethers::core::types::U256);
    ///`CumulativeLockInfo(address,address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CumulativeLockInfo {
        pub token: ::ethers::core::types::Address,
        pub factory: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///`Lock(uint256,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,string)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Lock {
        pub id: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub lock_date: ::ethers::core::types::U256,
        pub tge_date: ::ethers::core::types::U256,
        pub tge_bps: ::ethers::core::types::U256,
        pub cycle: ::ethers::core::types::U256,
        pub cycle_bps: ::ethers::core::types::U256,
        pub unlocked_amount: ::ethers::core::types::U256,
        pub description: ::std::string::String,
    }
}
