pub use trust_swap_lp_locker::*;
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
pub mod trust_swap_lp_locker {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NFT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("NFT"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addTokenToFreeList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addTokenToFreeList"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allDepositIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allDepositIds"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                (
                    ::std::borrow::ToOwned::to_owned("companyWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("companyWallet"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address payable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositId"),
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
                    ::std::borrow::ToOwned::to_owned("depositsByWithdrawalAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositsByWithdrawalAddress",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("extendLockDuration"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("extendLockDuration"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_unlockTime"),
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
                    ::std::borrow::ToOwned::to_owned("feesInUSD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feesInUSD"),
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
                    ::std::borrow::ToOwned::to_owned("getAllDepositIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllDepositIds"),
                        inputs: ::std::vec![],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDepositDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDepositDetails"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawalAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_isNFT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_migratedLockDepositId",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_isNFTMinted"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDepositsByWithdrawalAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDepositsByWithdrawalAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_withdrawalAddress",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFeesInETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFeesInETH"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalTokenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalTokenBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isFreeToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isFreeToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("listMigratedDepositIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("listMigratedDepositIds",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                (
                    ::std::borrow::ToOwned::to_owned("lockNFT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockNFT"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawalAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_mintNFT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("referrer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_withdrawalAddress",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_mintNFT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("referrer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockedNFTs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockedNFTs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
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
                    ::std::borrow::ToOwned::to_owned("lockedToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockedToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mintNFTforLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mintNFTforLock"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_id"),
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
                    ::std::borrow::ToOwned::to_owned("nftMinted"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nftMinted"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonfungiblePositionManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonfungiblePositionManager",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IERC721Enumerable",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pause"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("paused"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("priceEstimator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("priceEstimator"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPriceEstimator"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("referralDiscount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("referralDiscount"),
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
                    ::std::borrow::ToOwned::to_owned("referrerCut"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("referrerCut"),
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
                    ::std::borrow::ToOwned::to_owned("removeTokenFromFreeList"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeTokenFromFreeList",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setCompanyWallet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setCompanyWallet"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_companyWallet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address payable"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFeeParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeParams"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_priceEstimator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_usdTokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_feesInUSD"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_companyWallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address payable"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFeesInUSD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeesInUSD"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_feesInUSD"),
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
                    ::std::borrow::ToOwned::to_owned("setNFTContract"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setNFTContract"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_nftContractAddress",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setReferralParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setReferralParams"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_referralDiscount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_referrerCut"),
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
                    ::std::borrow::ToOwned::to_owned("splitLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("splitLock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_splitAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_splitUnlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_mintNFT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_splitLockId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferLocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferLocks"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_receiverAddress"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unpause"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateWhitelist"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateWhitelist"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("noFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateWhitelistAdminAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateWhitelistAdminAccess",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("access"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("usdTokenAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("usdTokenAddress"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v3Migrator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("v3Migrator"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IV3Migrator"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("walletTokenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("walletTokenBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("whitelistAdmins"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistAdmins"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("whitelistedWallets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistedWallets"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawTokens"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_amount"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CompanyWalletUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CompanyWalletUpdated",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("companyWallet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositNFT"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DepositNFT"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeesChanged"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("fees"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FreeTokenListUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FreeTokenListUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isFree"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockDurationExtended"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockDurationExtended",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("unlockTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockSplit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LockSplit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("remainingAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("splitLockId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newSplitLockAmount",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogNFTWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogNFTWithdrawal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LogTokenWithdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LogTokenWithdrawal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NftContractUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NftContractUpdated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("nftContract"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Paused"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReferralParamsChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ReferralParamsChanged",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("referralDiscount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("referrerCut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ReferrerRewarded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ReferrerRewarded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("addr"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("referrerCut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WhiteListAdminUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WhiteListAdminUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("status"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WhiteListUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WhiteListUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("wallet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("noFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
    pub static TRUSTSWAPLPLOCKER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct TrustSwapLpLocker<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TrustSwapLpLocker<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TrustSwapLpLocker<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TrustSwapLpLocker<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TrustSwapLpLocker<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TrustSwapLpLocker))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TrustSwapLpLocker<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TRUSTSWAPLPLOCKER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `NFT` (0x7c0b8de2) function
        pub fn nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([124, 11, 141, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addTokenToFreeList` (0xcd7d9e99) function
        pub fn add_token_to_free_list(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 125, 158, 153], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allDepositIds` (0xc9028aff) function
        pub fn all_deposit_ids(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 2, 138, 255], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `companyWallet` (0x1ec32d15) function
        pub fn company_wallet(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([30, 195, 45, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositId` (0x9852099c) function
        pub fn deposit_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([152, 82, 9, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositsByWithdrawalAddress` (0x530680d8) function
        pub fn deposits_by_withdrawal_address(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([83, 6, 128, 216], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extendLockDuration` (0x76704de0) function
        pub fn extend_lock_duration(
            &self,
            id: ::ethers::core::types::U256,
            unlock_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 112, 77, 224], (id, unlock_time))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feesInUSD` (0xaa182aef) function
        pub fn fees_in_usd(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([170, 24, 42, 239], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllDepositIds` (0x6ba03924) function
        pub fn get_all_deposit_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([107, 160, 57, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDepositDetails` (0x890db72f) function
        pub fn get_deposit_details(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([137, 13, 183, 47], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDepositsByWithdrawalAddress` (0x0bd59ad3) function
        pub fn get_deposits_by_withdrawal_address(
            &self,
            withdrawal_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([11, 213, 154, 211], withdrawal_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeesInETH` (0xfeeb733d) function
        pub fn get_fees_in_eth(
            &self,
            token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 235, 115, 61], token_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalTokenBalance` (0xadad19bd) function
        pub fn get_total_token_balance(
            &self,
            token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 173, 25, 189], token_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8129fc1c) function
        pub fn initialize(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isFreeToken` (0x31bff521) function
        pub fn is_free_token(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([49, 191, 245, 33], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `listMigratedDepositIds` (0x4d0925d3) function
        pub fn list_migrated_deposit_ids(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([77, 9, 37, 211], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockNFT` (0xe1b24aa2) function
        pub fn lock_nft(
            &self,
            token_address: ::ethers::core::types::Address,
            withdrawal_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            unlock_time: ::ethers::core::types::U256,
            token_id: ::ethers::core::types::U256,
            mint_nft: bool,
            referrer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [225, 178, 74, 162],
                    (
                        token_address,
                        withdrawal_address,
                        amount,
                        unlock_time,
                        token_id,
                        mint_nft,
                        referrer,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockToken` (0x5af06fed) function
        pub fn lock_token(
            &self,
            token_address: ::ethers::core::types::Address,
            withdrawal_address: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            unlock_time: ::ethers::core::types::U256,
            mint_nft: bool,
            referrer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [90, 240, 111, 237],
                    (
                        token_address,
                        withdrawal_address,
                        amount,
                        unlock_time,
                        mint_nft,
                        referrer,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockedNFTs` (0x945633c1) function
        pub fn locked_nf_ts(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([148, 86, 51, 193], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockedToken` (0xbb941cff) function
        pub fn locked_token(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([187, 148, 28, 255], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintNFTforLock` (0xee66beef) function
        pub fn mint_nf_tfor_lock(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 102, 190, 239], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nftMinted` (0xffd68f15) function
        pub fn nft_minted(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([255, 214, 143, 21], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonfungiblePositionManager` (0xb44a2722) function
        pub fn nonfungible_position_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([180, 74, 39, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceEstimator` (0xe3f1bc2b) function
        pub fn price_estimator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([227, 241, 188, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `referralDiscount` (0x21721b17) function
        pub fn referral_discount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([33, 114, 27, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `referrerCut` (0x4e1a5a7f) function
        pub fn referrer_cut(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 26, 90, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeTokenFromFreeList` (0xd8ad1b2b) function
        pub fn remove_token_from_free_list(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 173, 27, 43], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCompanyWallet` (0x28831187) function
        pub fn set_company_wallet(
            &self,
            company_wallet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 131, 17, 135], company_wallet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeParams` (0x6e8fa91d) function
        pub fn set_fee_params(
            &self,
            price_estimator: ::ethers::core::types::Address,
            usd_token_address: ::ethers::core::types::Address,
            fees_in_usd: ::ethers::core::types::U256,
            company_wallet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [110, 143, 169, 29],
                    (
                        price_estimator,
                        usd_token_address,
                        fees_in_usd,
                        company_wallet,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeesInUSD` (0xc8e6aa98) function
        pub fn set_fees_in_usd(
            &self,
            fees_in_usd: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 230, 170, 152], fees_in_usd)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNFTContract` (0xa7ccabdf) function
        pub fn set_nft_contract(
            &self,
            nft_contract_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 204, 171, 223], nft_contract_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setReferralParams` (0x7bea730e) function
        pub fn set_referral_params(
            &self,
            referral_discount: ::ethers::core::types::U256,
            referrer_cut: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 234, 115, 14], (referral_discount, referrer_cut))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `splitLock` (0x047bcc70) function
        pub fn split_lock(
            &self,
            id: ::ethers::core::types::U256,
            split_amount: ::ethers::core::types::U256,
            split_unlock_time: ::ethers::core::types::U256,
            mint_nft: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [4, 123, 204, 112],
                    (id, split_amount, split_unlock_time, mint_nft),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferLocks` (0x4c5f7f54) function
        pub fn transfer_locks(
            &self,
            id: ::ethers::core::types::U256,
            receiver_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 95, 127, 84], (id, receiver_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateWhitelist` (0x0d392cd9) function
        pub fn update_whitelist(
            &self,
            wallet: ::ethers::core::types::Address,
            no_fee: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 57, 44, 217], (wallet, no_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateWhitelistAdminAccess` (0xf8a55f9d) function
        pub fn update_whitelist_admin_access(
            &self,
            account: ::ethers::core::types::Address,
            access: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 165, 95, 157], (account, access))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usdTokenAddress` (0x3eac8dac) function
        pub fn usd_token_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([62, 172, 141, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v3Migrator` (0x6cbc1e9b) function
        pub fn v_3_migrator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([108, 188, 30, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `walletTokenBalance` (0xb9e7df1c) function
        pub fn wallet_token_balance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 231, 223, 28], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistAdmins` (0xd29dd76d) function
        pub fn whitelist_admins(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([210, 157, 215, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistedWallets` (0xa80dcfee) function
        pub fn whitelisted_wallets(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([168, 13, 207, 238], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawTokens` (0xba7bd2aa) function
        pub fn withdraw_tokens(
            &self,
            id: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 123, 210, 170], (id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CompanyWalletUpdated` event
        pub fn company_wallet_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CompanyWalletUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositNFT` event
        pub fn deposit_nft_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositNFTFilter> {
            self.0.event()
        }
        ///Gets the contract's `FeesChanged` event
        pub fn fees_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeesChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FreeTokenListUpdated` event
        pub fn free_token_list_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FreeTokenListUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LockDurationExtended` event
        pub fn lock_duration_extended_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockDurationExtendedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LockSplit` event
        pub fn lock_split_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LockSplitFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogNFTWithdrawal` event
        pub fn log_nft_withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNFTWithdrawalFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LogTokenWithdrawal` event
        pub fn log_token_withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogTokenWithdrawalFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NftContractUpdated` event
        pub fn nft_contract_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NftContractUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `ReferralParamsChanged` event
        pub fn referral_params_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ReferralParamsChangedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ReferrerRewarded` event
        pub fn referrer_rewarded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ReferrerRewardedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `WhiteListAdminUpdated` event
        pub fn white_list_admin_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WhiteListAdminUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `WhiteListUpdated` event
        pub fn white_list_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WhiteListUpdatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TrustSwapLpLockerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for TrustSwapLpLocker<M>
    {
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
    #[ethevent(name = "CompanyWalletUpdated", abi = "CompanyWalletUpdated(address)")]
    pub struct CompanyWalletUpdatedFilter {
        pub company_wallet: ::ethers::core::types::Address,
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
        name = "Deposit",
        abi = "Deposit(uint256,address,address,uint256,uint256)"
    )]
    pub struct DepositFilter {
        pub id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub withdrawal_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
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
        name = "DepositNFT",
        abi = "DepositNFT(uint256,address,uint256,address,uint256,uint256)"
    )]
    pub struct DepositNFTFilter {
        pub id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_address: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub withdrawal_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
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
    #[ethevent(name = "FeesChanged", abi = "FeesChanged(uint256)")]
    pub struct FeesChangedFilter {
        #[ethevent(indexed)]
        pub fees: ::ethers::core::types::U256,
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
        name = "FreeTokenListUpdated",
        abi = "FreeTokenListUpdated(address,bool)"
    )]
    pub struct FreeTokenListUpdatedFilter {
        pub token: ::ethers::core::types::Address,
        pub is_free: bool,
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
        name = "LockDurationExtended",
        abi = "LockDurationExtended(uint256,uint256)"
    )]
    pub struct LockDurationExtendedFilter {
        pub id: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
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
    #[ethevent(name = "LockSplit", abi = "LockSplit(uint256,uint256,uint256,uint256)")]
    pub struct LockSplitFilter {
        pub id: ::ethers::core::types::U256,
        pub remaining_amount: ::ethers::core::types::U256,
        pub split_lock_id: ::ethers::core::types::U256,
        pub new_split_lock_amount: ::ethers::core::types::U256,
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
        name = "LogNFTWithdrawal",
        abi = "LogNFTWithdrawal(uint256,address,uint256,address,uint256)"
    )]
    pub struct LogNFTWithdrawalFilter {
        pub id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_address: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub withdrawal_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "LogTokenWithdrawal",
        abi = "LogTokenWithdrawal(uint256,address,address,uint256)"
    )]
    pub struct LogTokenWithdrawalFilter {
        pub id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub withdrawal_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "NftContractUpdated", abi = "NftContractUpdated(address)")]
    pub struct NftContractUpdatedFilter {
        pub nft_contract: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ::ethers::core::types::Address,
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
        name = "ReferralParamsChanged",
        abi = "ReferralParamsChanged(uint256,uint256)"
    )]
    pub struct ReferralParamsChangedFilter {
        pub referral_discount: ::ethers::core::types::U256,
        pub referrer_cut: ::ethers::core::types::U256,
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
    #[ethevent(name = "ReferrerRewarded", abi = "ReferrerRewarded(address,uint256)")]
    pub struct ReferrerRewardedFilter {
        #[ethevent(indexed)]
        pub addr: ::ethers::core::types::Address,
        pub referrer_cut: ::ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ::ethers::core::types::Address,
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
        name = "WhiteListAdminUpdated",
        abi = "WhiteListAdminUpdated(address,bool)"
    )]
    pub struct WhiteListAdminUpdatedFilter {
        pub wallet: ::ethers::core::types::Address,
        pub status: bool,
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
    #[ethevent(name = "WhiteListUpdated", abi = "WhiteListUpdated(address,bool)")]
    pub struct WhiteListUpdatedFilter {
        pub wallet: ::ethers::core::types::Address,
        pub no_fee: bool,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TrustSwapLpLockerEvents {
        CompanyWalletUpdatedFilter(CompanyWalletUpdatedFilter),
        DepositFilter(DepositFilter),
        DepositNFTFilter(DepositNFTFilter),
        FeesChangedFilter(FeesChangedFilter),
        FreeTokenListUpdatedFilter(FreeTokenListUpdatedFilter),
        LockDurationExtendedFilter(LockDurationExtendedFilter),
        LockSplitFilter(LockSplitFilter),
        LogNFTWithdrawalFilter(LogNFTWithdrawalFilter),
        LogTokenWithdrawalFilter(LogTokenWithdrawalFilter),
        NftContractUpdatedFilter(NftContractUpdatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        ReferralParamsChangedFilter(ReferralParamsChangedFilter),
        ReferrerRewardedFilter(ReferrerRewardedFilter),
        UnpausedFilter(UnpausedFilter),
        WhiteListAdminUpdatedFilter(WhiteListAdminUpdatedFilter),
        WhiteListUpdatedFilter(WhiteListUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TrustSwapLpLockerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CompanyWalletUpdatedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::CompanyWalletUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositNFTFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::DepositNFTFilter(decoded));
            }
            if let Ok(decoded) = FeesChangedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::FeesChangedFilter(decoded));
            }
            if let Ok(decoded) = FreeTokenListUpdatedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::FreeTokenListUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LockDurationExtendedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::LockDurationExtendedFilter(decoded));
            }
            if let Ok(decoded) = LockSplitFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::LockSplitFilter(decoded));
            }
            if let Ok(decoded) = LogNFTWithdrawalFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::LogNFTWithdrawalFilter(decoded));
            }
            if let Ok(decoded) = LogTokenWithdrawalFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::LogTokenWithdrawalFilter(decoded));
            }
            if let Ok(decoded) = NftContractUpdatedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::NftContractUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = ReferralParamsChangedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::ReferralParamsChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReferrerRewardedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::ReferrerRewardedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WhiteListAdminUpdatedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::WhiteListAdminUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = WhiteListUpdatedFilter::decode_log(log) {
                return Ok(TrustSwapLpLockerEvents::WhiteListUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TrustSwapLpLockerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CompanyWalletUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositNFTFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreeTokenListUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockDurationExtendedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockSplitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNFTWithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogTokenWithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NftContractUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReferralParamsChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReferrerRewardedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhiteListAdminUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhiteListUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CompanyWalletUpdatedFilter> for TrustSwapLpLockerEvents {
        fn from(value: CompanyWalletUpdatedFilter) -> Self {
            Self::CompanyWalletUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for TrustSwapLpLockerEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<DepositNFTFilter> for TrustSwapLpLockerEvents {
        fn from(value: DepositNFTFilter) -> Self {
            Self::DepositNFTFilter(value)
        }
    }
    impl ::core::convert::From<FeesChangedFilter> for TrustSwapLpLockerEvents {
        fn from(value: FeesChangedFilter) -> Self {
            Self::FeesChangedFilter(value)
        }
    }
    impl ::core::convert::From<FreeTokenListUpdatedFilter> for TrustSwapLpLockerEvents {
        fn from(value: FreeTokenListUpdatedFilter) -> Self {
            Self::FreeTokenListUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LockDurationExtendedFilter> for TrustSwapLpLockerEvents {
        fn from(value: LockDurationExtendedFilter) -> Self {
            Self::LockDurationExtendedFilter(value)
        }
    }
    impl ::core::convert::From<LockSplitFilter> for TrustSwapLpLockerEvents {
        fn from(value: LockSplitFilter) -> Self {
            Self::LockSplitFilter(value)
        }
    }
    impl ::core::convert::From<LogNFTWithdrawalFilter> for TrustSwapLpLockerEvents {
        fn from(value: LogNFTWithdrawalFilter) -> Self {
            Self::LogNFTWithdrawalFilter(value)
        }
    }
    impl ::core::convert::From<LogTokenWithdrawalFilter> for TrustSwapLpLockerEvents {
        fn from(value: LogTokenWithdrawalFilter) -> Self {
            Self::LogTokenWithdrawalFilter(value)
        }
    }
    impl ::core::convert::From<NftContractUpdatedFilter> for TrustSwapLpLockerEvents {
        fn from(value: NftContractUpdatedFilter) -> Self {
            Self::NftContractUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for TrustSwapLpLockerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for TrustSwapLpLockerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<ReferralParamsChangedFilter> for TrustSwapLpLockerEvents {
        fn from(value: ReferralParamsChangedFilter) -> Self {
            Self::ReferralParamsChangedFilter(value)
        }
    }
    impl ::core::convert::From<ReferrerRewardedFilter> for TrustSwapLpLockerEvents {
        fn from(value: ReferrerRewardedFilter) -> Self {
            Self::ReferrerRewardedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for TrustSwapLpLockerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    impl ::core::convert::From<WhiteListAdminUpdatedFilter> for TrustSwapLpLockerEvents {
        fn from(value: WhiteListAdminUpdatedFilter) -> Self {
            Self::WhiteListAdminUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<WhiteListUpdatedFilter> for TrustSwapLpLockerEvents {
        fn from(value: WhiteListUpdatedFilter) -> Self {
            Self::WhiteListUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `NFT` function with signature `NFT()` and selector `0x7c0b8de2`
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
    #[ethcall(name = "NFT", abi = "NFT()")]
    pub struct NftCall;
    ///Container type for all input parameters for the `addTokenToFreeList` function with signature `addTokenToFreeList(address)` and selector `0xcd7d9e99`
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
    #[ethcall(name = "addTokenToFreeList", abi = "addTokenToFreeList(address)")]
    pub struct AddTokenToFreeListCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allDepositIds` function with signature `allDepositIds(uint256)` and selector `0xc9028aff`
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
    #[ethcall(name = "allDepositIds", abi = "allDepositIds(uint256)")]
    pub struct AllDepositIdsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `companyWallet` function with signature `companyWallet()` and selector `0x1ec32d15`
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
    #[ethcall(name = "companyWallet", abi = "companyWallet()")]
    pub struct CompanyWalletCall;
    ///Container type for all input parameters for the `depositId` function with signature `depositId()` and selector `0x9852099c`
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
    #[ethcall(name = "depositId", abi = "depositId()")]
    pub struct DepositIdCall;
    ///Container type for all input parameters for the `depositsByWithdrawalAddress` function with signature `depositsByWithdrawalAddress(address,uint256)` and selector `0x530680d8`
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
        name = "depositsByWithdrawalAddress",
        abi = "depositsByWithdrawalAddress(address,uint256)"
    )]
    pub struct DepositsByWithdrawalAddressCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `extendLockDuration` function with signature `extendLockDuration(uint256,uint256)` and selector `0x76704de0`
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
        name = "extendLockDuration",
        abi = "extendLockDuration(uint256,uint256)"
    )]
    pub struct ExtendLockDurationCall {
        pub id: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `feesInUSD` function with signature `feesInUSD()` and selector `0xaa182aef`
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
    #[ethcall(name = "feesInUSD", abi = "feesInUSD()")]
    pub struct FeesInUSDCall;
    ///Container type for all input parameters for the `getAllDepositIds` function with signature `getAllDepositIds()` and selector `0x6ba03924`
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
    #[ethcall(name = "getAllDepositIds", abi = "getAllDepositIds()")]
    pub struct GetAllDepositIdsCall;
    ///Container type for all input parameters for the `getDepositDetails` function with signature `getDepositDetails(uint256)` and selector `0x890db72f`
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
    #[ethcall(name = "getDepositDetails", abi = "getDepositDetails(uint256)")]
    pub struct GetDepositDetailsCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getDepositsByWithdrawalAddress` function with signature `getDepositsByWithdrawalAddress(address)` and selector `0x0bd59ad3`
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
        name = "getDepositsByWithdrawalAddress",
        abi = "getDepositsByWithdrawalAddress(address)"
    )]
    pub struct GetDepositsByWithdrawalAddressCall {
        pub withdrawal_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getFeesInETH` function with signature `getFeesInETH(address)` and selector `0xfeeb733d`
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
    #[ethcall(name = "getFeesInETH", abi = "getFeesInETH(address)")]
    pub struct GetFeesInETHCall {
        pub token_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getTotalTokenBalance` function with signature `getTotalTokenBalance(address)` and selector `0xadad19bd`
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
    #[ethcall(name = "getTotalTokenBalance", abi = "getTotalTokenBalance(address)")]
    pub struct GetTotalTokenBalanceCall {
        pub token_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize()` and selector `0x8129fc1c`
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
    #[ethcall(name = "initialize", abi = "initialize()")]
    pub struct InitializeCall;
    ///Container type for all input parameters for the `isFreeToken` function with signature `isFreeToken(address)` and selector `0x31bff521`
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
    #[ethcall(name = "isFreeToken", abi = "isFreeToken(address)")]
    pub struct IsFreeTokenCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `listMigratedDepositIds` function with signature `listMigratedDepositIds(uint256)` and selector `0x4d0925d3`
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
        name = "listMigratedDepositIds",
        abi = "listMigratedDepositIds(uint256)"
    )]
    pub struct ListMigratedDepositIdsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `lockNFT` function with signature `lockNFT(address,address,uint256,uint256,uint256,bool,address)` and selector `0xe1b24aa2`
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
        name = "lockNFT",
        abi = "lockNFT(address,address,uint256,uint256,uint256,bool,address)"
    )]
    pub struct LockNFTCall {
        pub token_address: ::ethers::core::types::Address,
        pub withdrawal_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
        pub mint_nft: bool,
        pub referrer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lockToken` function with signature `lockToken(address,address,uint256,uint256,bool,address)` and selector `0x5af06fed`
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
        name = "lockToken",
        abi = "lockToken(address,address,uint256,uint256,bool,address)"
    )]
    pub struct LockTokenCall {
        pub token_address: ::ethers::core::types::Address,
        pub withdrawal_address: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
        pub mint_nft: bool,
        pub referrer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lockedNFTs` function with signature `lockedNFTs(uint256)` and selector `0x945633c1`
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
    #[ethcall(name = "lockedNFTs", abi = "lockedNFTs(uint256)")]
    pub struct LockedNFTsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `lockedToken` function with signature `lockedToken(uint256)` and selector `0xbb941cff`
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
    #[ethcall(name = "lockedToken", abi = "lockedToken(uint256)")]
    pub struct LockedTokenCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `mintNFTforLock` function with signature `mintNFTforLock(uint256)` and selector `0xee66beef`
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
    #[ethcall(name = "mintNFTforLock", abi = "mintNFTforLock(uint256)")]
    pub struct MintNFTforLockCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `nftMinted` function with signature `nftMinted(uint256)` and selector `0xffd68f15`
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
    #[ethcall(name = "nftMinted", abi = "nftMinted(uint256)")]
    pub struct NftMintedCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `nonfungiblePositionManager` function with signature `nonfungiblePositionManager()` and selector `0xb44a2722`
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
        name = "nonfungiblePositionManager",
        abi = "nonfungiblePositionManager()"
    )]
    pub struct NonfungiblePositionManagerCall;
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `priceEstimator` function with signature `priceEstimator()` and selector `0xe3f1bc2b`
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
    #[ethcall(name = "priceEstimator", abi = "priceEstimator()")]
    pub struct PriceEstimatorCall;
    ///Container type for all input parameters for the `referralDiscount` function with signature `referralDiscount()` and selector `0x21721b17`
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
    #[ethcall(name = "referralDiscount", abi = "referralDiscount()")]
    pub struct ReferralDiscountCall;
    ///Container type for all input parameters for the `referrerCut` function with signature `referrerCut()` and selector `0x4e1a5a7f`
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
    #[ethcall(name = "referrerCut", abi = "referrerCut()")]
    pub struct ReferrerCutCall;
    ///Container type for all input parameters for the `removeTokenFromFreeList` function with signature `removeTokenFromFreeList(address)` and selector `0xd8ad1b2b`
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
        name = "removeTokenFromFreeList",
        abi = "removeTokenFromFreeList(address)"
    )]
    pub struct RemoveTokenFromFreeListCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setCompanyWallet` function with signature `setCompanyWallet(address)` and selector `0x28831187`
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
    #[ethcall(name = "setCompanyWallet", abi = "setCompanyWallet(address)")]
    pub struct SetCompanyWalletCall {
        pub company_wallet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeParams` function with signature `setFeeParams(address,address,uint256,address)` and selector `0x6e8fa91d`
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
        name = "setFeeParams",
        abi = "setFeeParams(address,address,uint256,address)"
    )]
    pub struct SetFeeParamsCall {
        pub price_estimator: ::ethers::core::types::Address,
        pub usd_token_address: ::ethers::core::types::Address,
        pub fees_in_usd: ::ethers::core::types::U256,
        pub company_wallet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeesInUSD` function with signature `setFeesInUSD(uint256)` and selector `0xc8e6aa98`
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
    #[ethcall(name = "setFeesInUSD", abi = "setFeesInUSD(uint256)")]
    pub struct SetFeesInUSDCall {
        pub fees_in_usd: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setNFTContract` function with signature `setNFTContract(address)` and selector `0xa7ccabdf`
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
    #[ethcall(name = "setNFTContract", abi = "setNFTContract(address)")]
    pub struct SetNFTContractCall {
        pub nft_contract_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setReferralParams` function with signature `setReferralParams(uint256,uint256)` and selector `0x7bea730e`
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
    #[ethcall(name = "setReferralParams", abi = "setReferralParams(uint256,uint256)")]
    pub struct SetReferralParamsCall {
        pub referral_discount: ::ethers::core::types::U256,
        pub referrer_cut: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `splitLock` function with signature `splitLock(uint256,uint256,uint256,bool)` and selector `0x047bcc70`
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
    #[ethcall(name = "splitLock", abi = "splitLock(uint256,uint256,uint256,bool)")]
    pub struct SplitLockCall {
        pub id: ::ethers::core::types::U256,
        pub split_amount: ::ethers::core::types::U256,
        pub split_unlock_time: ::ethers::core::types::U256,
        pub mint_nft: bool,
    }
    ///Container type for all input parameters for the `transferLocks` function with signature `transferLocks(uint256,address)` and selector `0x4c5f7f54`
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
    #[ethcall(name = "transferLocks", abi = "transferLocks(uint256,address)")]
    pub struct TransferLocksCall {
        pub id: ::ethers::core::types::U256,
        pub receiver_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `updateWhitelist` function with signature `updateWhitelist(address,bool)` and selector `0x0d392cd9`
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
    #[ethcall(name = "updateWhitelist", abi = "updateWhitelist(address,bool)")]
    pub struct UpdateWhitelistCall {
        pub wallet: ::ethers::core::types::Address,
        pub no_fee: bool,
    }
    ///Container type for all input parameters for the `updateWhitelistAdminAccess` function with signature `updateWhitelistAdminAccess(address,bool)` and selector `0xf8a55f9d`
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
        name = "updateWhitelistAdminAccess",
        abi = "updateWhitelistAdminAccess(address,bool)"
    )]
    pub struct UpdateWhitelistAdminAccessCall {
        pub account: ::ethers::core::types::Address,
        pub access: bool,
    }
    ///Container type for all input parameters for the `usdTokenAddress` function with signature `usdTokenAddress()` and selector `0x3eac8dac`
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
    #[ethcall(name = "usdTokenAddress", abi = "usdTokenAddress()")]
    pub struct UsdTokenAddressCall;
    ///Container type for all input parameters for the `v3Migrator` function with signature `v3Migrator()` and selector `0x6cbc1e9b`
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
    #[ethcall(name = "v3Migrator", abi = "v3Migrator()")]
    pub struct V3MigratorCall;
    ///Container type for all input parameters for the `walletTokenBalance` function with signature `walletTokenBalance(address,address)` and selector `0xb9e7df1c`
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
        name = "walletTokenBalance",
        abi = "walletTokenBalance(address,address)"
    )]
    pub struct WalletTokenBalanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `whitelistAdmins` function with signature `whitelistAdmins(address)` and selector `0xd29dd76d`
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
    #[ethcall(name = "whitelistAdmins", abi = "whitelistAdmins(address)")]
    pub struct WhitelistAdminsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `whitelistedWallets` function with signature `whitelistedWallets(address)` and selector `0xa80dcfee`
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
    #[ethcall(name = "whitelistedWallets", abi = "whitelistedWallets(address)")]
    pub struct WhitelistedWalletsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdrawTokens` function with signature `withdrawTokens(uint256,uint256)` and selector `0xba7bd2aa`
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
    #[ethcall(name = "withdrawTokens", abi = "withdrawTokens(uint256,uint256)")]
    pub struct WithdrawTokensCall {
        pub id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TrustSwapLpLockerCalls {
        Nft(NftCall),
        AddTokenToFreeList(AddTokenToFreeListCall),
        AllDepositIds(AllDepositIdsCall),
        CompanyWallet(CompanyWalletCall),
        DepositId(DepositIdCall),
        DepositsByWithdrawalAddress(DepositsByWithdrawalAddressCall),
        ExtendLockDuration(ExtendLockDurationCall),
        FeesInUSD(FeesInUSDCall),
        GetAllDepositIds(GetAllDepositIdsCall),
        GetDepositDetails(GetDepositDetailsCall),
        GetDepositsByWithdrawalAddress(GetDepositsByWithdrawalAddressCall),
        GetFeesInETH(GetFeesInETHCall),
        GetTotalTokenBalance(GetTotalTokenBalanceCall),
        Initialize(InitializeCall),
        IsFreeToken(IsFreeTokenCall),
        ListMigratedDepositIds(ListMigratedDepositIdsCall),
        LockNFT(LockNFTCall),
        LockToken(LockTokenCall),
        LockedNFTs(LockedNFTsCall),
        LockedToken(LockedTokenCall),
        MintNFTforLock(MintNFTforLockCall),
        NftMinted(NftMintedCall),
        NonfungiblePositionManager(NonfungiblePositionManagerCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Paused(PausedCall),
        PriceEstimator(PriceEstimatorCall),
        ReferralDiscount(ReferralDiscountCall),
        ReferrerCut(ReferrerCutCall),
        RemoveTokenFromFreeList(RemoveTokenFromFreeListCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetCompanyWallet(SetCompanyWalletCall),
        SetFeeParams(SetFeeParamsCall),
        SetFeesInUSD(SetFeesInUSDCall),
        SetNFTContract(SetNFTContractCall),
        SetReferralParams(SetReferralParamsCall),
        SplitLock(SplitLockCall),
        TransferLocks(TransferLocksCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateWhitelist(UpdateWhitelistCall),
        UpdateWhitelistAdminAccess(UpdateWhitelistAdminAccessCall),
        UsdTokenAddress(UsdTokenAddressCall),
        V3Migrator(V3MigratorCall),
        WalletTokenBalance(WalletTokenBalanceCall),
        WhitelistAdmins(WhitelistAdminsCall),
        WhitelistedWallets(WhitelistedWalletsCall),
        WithdrawTokens(WithdrawTokensCall),
    }
    impl ::ethers::core::abi::AbiDecode for TrustSwapLpLockerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <NftCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nft(decoded));
            }
            if let Ok(decoded) =
                <AddTokenToFreeListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddTokenToFreeList(decoded));
            }
            if let Ok(decoded) = <AllDepositIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllDepositIds(decoded));
            }
            if let Ok(decoded) = <CompanyWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompanyWallet(decoded));
            }
            if let Ok(decoded) = <DepositIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositId(decoded));
            }
            if let Ok(decoded) =
                <DepositsByWithdrawalAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositsByWithdrawalAddress(decoded));
            }
            if let Ok(decoded) =
                <ExtendLockDurationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExtendLockDuration(decoded));
            }
            if let Ok(decoded) = <FeesInUSDCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeesInUSD(decoded));
            }
            if let Ok(decoded) =
                <GetAllDepositIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllDepositIds(decoded));
            }
            if let Ok(decoded) =
                <GetDepositDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDepositDetails(decoded));
            }
            if let Ok(decoded) =
                <GetDepositsByWithdrawalAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDepositsByWithdrawalAddress(decoded));
            }
            if let Ok(decoded) = <GetFeesInETHCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFeesInETH(decoded));
            }
            if let Ok(decoded) =
                <GetTotalTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTotalTokenBalance(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsFreeTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsFreeToken(decoded));
            }
            if let Ok(decoded) =
                <ListMigratedDepositIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ListMigratedDepositIds(decoded));
            }
            if let Ok(decoded) = <LockNFTCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockNFT(decoded));
            }
            if let Ok(decoded) = <LockTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockToken(decoded));
            }
            if let Ok(decoded) = <LockedNFTsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockedNFTs(decoded));
            }
            if let Ok(decoded) = <LockedTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockedToken(decoded));
            }
            if let Ok(decoded) =
                <MintNFTforLockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MintNFTforLock(decoded));
            }
            if let Ok(decoded) = <NftMintedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NftMinted(decoded));
            }
            if let Ok(decoded) =
                <NonfungiblePositionManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NonfungiblePositionManager(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <PriceEstimatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PriceEstimator(decoded));
            }
            if let Ok(decoded) =
                <ReferralDiscountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReferralDiscount(decoded));
            }
            if let Ok(decoded) = <ReferrerCutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReferrerCut(decoded));
            }
            if let Ok(decoded) =
                <RemoveTokenFromFreeListCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveTokenFromFreeList(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetCompanyWalletCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetCompanyWallet(decoded));
            }
            if let Ok(decoded) = <SetFeeParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeParams(decoded));
            }
            if let Ok(decoded) = <SetFeesInUSDCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeesInUSD(decoded));
            }
            if let Ok(decoded) =
                <SetNFTContractCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetNFTContract(decoded));
            }
            if let Ok(decoded) =
                <SetReferralParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetReferralParams(decoded));
            }
            if let Ok(decoded) = <SplitLockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SplitLock(decoded));
            }
            if let Ok(decoded) = <TransferLocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferLocks(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateWhitelistCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateWhitelist(decoded));
            }
            if let Ok(decoded) =
                <UpdateWhitelistAdminAccessCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateWhitelistAdminAccess(decoded));
            }
            if let Ok(decoded) =
                <UsdTokenAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UsdTokenAddress(decoded));
            }
            if let Ok(decoded) = <V3MigratorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::V3Migrator(decoded));
            }
            if let Ok(decoded) =
                <WalletTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WalletTokenBalance(decoded));
            }
            if let Ok(decoded) =
                <WhitelistAdminsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistAdmins(decoded));
            }
            if let Ok(decoded) =
                <WhitelistedWalletsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistedWallets(decoded));
            }
            if let Ok(decoded) =
                <WithdrawTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawTokens(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TrustSwapLpLockerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Nft(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddTokenToFreeList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllDepositIds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CompanyWallet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositsByWithdrawalAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExtendLockDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeesInUSD(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllDepositIds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositDetails(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDepositsByWithdrawalAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFeesInETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTotalTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsFreeToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ListMigratedDepositIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockNFT(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockedNFTs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockedToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintNFTforLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NftMinted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonfungiblePositionManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PriceEstimator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReferralDiscount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReferrerCut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveTokenFromFreeList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCompanyWallet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeesInUSD(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetNFTContract(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetReferralParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SplitLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferLocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateWhitelist(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateWhitelistAdminAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsdTokenAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::V3Migrator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WalletTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistAdmins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WhitelistedWallets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TrustSwapLpLockerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Nft(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddTokenToFreeList(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllDepositIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompanyWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositId(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositsByWithdrawalAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtendLockDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesInUSD(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllDepositIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositsByWithdrawalAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetFeesInETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsFreeToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::ListMigratedDepositIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockNFT(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockedNFTs(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockedToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintNFTforLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::NftMinted(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonfungiblePositionManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceEstimator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReferralDiscount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReferrerCut(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveTokenFromFreeList(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCompanyWallet(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeesInUSD(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNFTContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetReferralParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SplitLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferLocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateWhitelist(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateWhitelistAdminAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsdTokenAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3Migrator(element) => ::core::fmt::Display::fmt(element, f),
                Self::WalletTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistAdmins(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistedWallets(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTokens(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NftCall> for TrustSwapLpLockerCalls {
        fn from(value: NftCall) -> Self {
            Self::Nft(value)
        }
    }
    impl ::core::convert::From<AddTokenToFreeListCall> for TrustSwapLpLockerCalls {
        fn from(value: AddTokenToFreeListCall) -> Self {
            Self::AddTokenToFreeList(value)
        }
    }
    impl ::core::convert::From<AllDepositIdsCall> for TrustSwapLpLockerCalls {
        fn from(value: AllDepositIdsCall) -> Self {
            Self::AllDepositIds(value)
        }
    }
    impl ::core::convert::From<CompanyWalletCall> for TrustSwapLpLockerCalls {
        fn from(value: CompanyWalletCall) -> Self {
            Self::CompanyWallet(value)
        }
    }
    impl ::core::convert::From<DepositIdCall> for TrustSwapLpLockerCalls {
        fn from(value: DepositIdCall) -> Self {
            Self::DepositId(value)
        }
    }
    impl ::core::convert::From<DepositsByWithdrawalAddressCall> for TrustSwapLpLockerCalls {
        fn from(value: DepositsByWithdrawalAddressCall) -> Self {
            Self::DepositsByWithdrawalAddress(value)
        }
    }
    impl ::core::convert::From<ExtendLockDurationCall> for TrustSwapLpLockerCalls {
        fn from(value: ExtendLockDurationCall) -> Self {
            Self::ExtendLockDuration(value)
        }
    }
    impl ::core::convert::From<FeesInUSDCall> for TrustSwapLpLockerCalls {
        fn from(value: FeesInUSDCall) -> Self {
            Self::FeesInUSD(value)
        }
    }
    impl ::core::convert::From<GetAllDepositIdsCall> for TrustSwapLpLockerCalls {
        fn from(value: GetAllDepositIdsCall) -> Self {
            Self::GetAllDepositIds(value)
        }
    }
    impl ::core::convert::From<GetDepositDetailsCall> for TrustSwapLpLockerCalls {
        fn from(value: GetDepositDetailsCall) -> Self {
            Self::GetDepositDetails(value)
        }
    }
    impl ::core::convert::From<GetDepositsByWithdrawalAddressCall> for TrustSwapLpLockerCalls {
        fn from(value: GetDepositsByWithdrawalAddressCall) -> Self {
            Self::GetDepositsByWithdrawalAddress(value)
        }
    }
    impl ::core::convert::From<GetFeesInETHCall> for TrustSwapLpLockerCalls {
        fn from(value: GetFeesInETHCall) -> Self {
            Self::GetFeesInETH(value)
        }
    }
    impl ::core::convert::From<GetTotalTokenBalanceCall> for TrustSwapLpLockerCalls {
        fn from(value: GetTotalTokenBalanceCall) -> Self {
            Self::GetTotalTokenBalance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for TrustSwapLpLockerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsFreeTokenCall> for TrustSwapLpLockerCalls {
        fn from(value: IsFreeTokenCall) -> Self {
            Self::IsFreeToken(value)
        }
    }
    impl ::core::convert::From<ListMigratedDepositIdsCall> for TrustSwapLpLockerCalls {
        fn from(value: ListMigratedDepositIdsCall) -> Self {
            Self::ListMigratedDepositIds(value)
        }
    }
    impl ::core::convert::From<LockNFTCall> for TrustSwapLpLockerCalls {
        fn from(value: LockNFTCall) -> Self {
            Self::LockNFT(value)
        }
    }
    impl ::core::convert::From<LockTokenCall> for TrustSwapLpLockerCalls {
        fn from(value: LockTokenCall) -> Self {
            Self::LockToken(value)
        }
    }
    impl ::core::convert::From<LockedNFTsCall> for TrustSwapLpLockerCalls {
        fn from(value: LockedNFTsCall) -> Self {
            Self::LockedNFTs(value)
        }
    }
    impl ::core::convert::From<LockedTokenCall> for TrustSwapLpLockerCalls {
        fn from(value: LockedTokenCall) -> Self {
            Self::LockedToken(value)
        }
    }
    impl ::core::convert::From<MintNFTforLockCall> for TrustSwapLpLockerCalls {
        fn from(value: MintNFTforLockCall) -> Self {
            Self::MintNFTforLock(value)
        }
    }
    impl ::core::convert::From<NftMintedCall> for TrustSwapLpLockerCalls {
        fn from(value: NftMintedCall) -> Self {
            Self::NftMinted(value)
        }
    }
    impl ::core::convert::From<NonfungiblePositionManagerCall> for TrustSwapLpLockerCalls {
        fn from(value: NonfungiblePositionManagerCall) -> Self {
            Self::NonfungiblePositionManager(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for TrustSwapLpLockerCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for TrustSwapLpLockerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for TrustSwapLpLockerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PausedCall> for TrustSwapLpLockerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PriceEstimatorCall> for TrustSwapLpLockerCalls {
        fn from(value: PriceEstimatorCall) -> Self {
            Self::PriceEstimator(value)
        }
    }
    impl ::core::convert::From<ReferralDiscountCall> for TrustSwapLpLockerCalls {
        fn from(value: ReferralDiscountCall) -> Self {
            Self::ReferralDiscount(value)
        }
    }
    impl ::core::convert::From<ReferrerCutCall> for TrustSwapLpLockerCalls {
        fn from(value: ReferrerCutCall) -> Self {
            Self::ReferrerCut(value)
        }
    }
    impl ::core::convert::From<RemoveTokenFromFreeListCall> for TrustSwapLpLockerCalls {
        fn from(value: RemoveTokenFromFreeListCall) -> Self {
            Self::RemoveTokenFromFreeList(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for TrustSwapLpLockerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetCompanyWalletCall> for TrustSwapLpLockerCalls {
        fn from(value: SetCompanyWalletCall) -> Self {
            Self::SetCompanyWallet(value)
        }
    }
    impl ::core::convert::From<SetFeeParamsCall> for TrustSwapLpLockerCalls {
        fn from(value: SetFeeParamsCall) -> Self {
            Self::SetFeeParams(value)
        }
    }
    impl ::core::convert::From<SetFeesInUSDCall> for TrustSwapLpLockerCalls {
        fn from(value: SetFeesInUSDCall) -> Self {
            Self::SetFeesInUSD(value)
        }
    }
    impl ::core::convert::From<SetNFTContractCall> for TrustSwapLpLockerCalls {
        fn from(value: SetNFTContractCall) -> Self {
            Self::SetNFTContract(value)
        }
    }
    impl ::core::convert::From<SetReferralParamsCall> for TrustSwapLpLockerCalls {
        fn from(value: SetReferralParamsCall) -> Self {
            Self::SetReferralParams(value)
        }
    }
    impl ::core::convert::From<SplitLockCall> for TrustSwapLpLockerCalls {
        fn from(value: SplitLockCall) -> Self {
            Self::SplitLock(value)
        }
    }
    impl ::core::convert::From<TransferLocksCall> for TrustSwapLpLockerCalls {
        fn from(value: TransferLocksCall) -> Self {
            Self::TransferLocks(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for TrustSwapLpLockerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for TrustSwapLpLockerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateWhitelistCall> for TrustSwapLpLockerCalls {
        fn from(value: UpdateWhitelistCall) -> Self {
            Self::UpdateWhitelist(value)
        }
    }
    impl ::core::convert::From<UpdateWhitelistAdminAccessCall> for TrustSwapLpLockerCalls {
        fn from(value: UpdateWhitelistAdminAccessCall) -> Self {
            Self::UpdateWhitelistAdminAccess(value)
        }
    }
    impl ::core::convert::From<UsdTokenAddressCall> for TrustSwapLpLockerCalls {
        fn from(value: UsdTokenAddressCall) -> Self {
            Self::UsdTokenAddress(value)
        }
    }
    impl ::core::convert::From<V3MigratorCall> for TrustSwapLpLockerCalls {
        fn from(value: V3MigratorCall) -> Self {
            Self::V3Migrator(value)
        }
    }
    impl ::core::convert::From<WalletTokenBalanceCall> for TrustSwapLpLockerCalls {
        fn from(value: WalletTokenBalanceCall) -> Self {
            Self::WalletTokenBalance(value)
        }
    }
    impl ::core::convert::From<WhitelistAdminsCall> for TrustSwapLpLockerCalls {
        fn from(value: WhitelistAdminsCall) -> Self {
            Self::WhitelistAdmins(value)
        }
    }
    impl ::core::convert::From<WhitelistedWalletsCall> for TrustSwapLpLockerCalls {
        fn from(value: WhitelistedWalletsCall) -> Self {
            Self::WhitelistedWallets(value)
        }
    }
    impl ::core::convert::From<WithdrawTokensCall> for TrustSwapLpLockerCalls {
        fn from(value: WithdrawTokensCall) -> Self {
            Self::WithdrawTokens(value)
        }
    }
    ///Container type for all return fields from the `NFT` function with signature `NFT()` and selector `0x7c0b8de2`
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
    pub struct NftReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allDepositIds` function with signature `allDepositIds(uint256)` and selector `0xc9028aff`
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
    pub struct AllDepositIdsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `companyWallet` function with signature `companyWallet()` and selector `0x1ec32d15`
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
    pub struct CompanyWalletReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `depositId` function with signature `depositId()` and selector `0x9852099c`
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
    pub struct DepositIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `depositsByWithdrawalAddress` function with signature `depositsByWithdrawalAddress(address,uint256)` and selector `0x530680d8`
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
    pub struct DepositsByWithdrawalAddressReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feesInUSD` function with signature `feesInUSD()` and selector `0xaa182aef`
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
    pub struct FeesInUSDReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAllDepositIds` function with signature `getAllDepositIds()` and selector `0x6ba03924`
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
    pub struct GetAllDepositIdsReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getDepositDetails` function with signature `getDepositDetails(uint256)` and selector `0x890db72f`
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
    pub struct GetDepositDetailsReturn {
        pub token_address: ::ethers::core::types::Address,
        pub withdrawal_address: ::ethers::core::types::Address,
        pub token_amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
        pub withdrawn: bool,
        pub token_id: ::ethers::core::types::U256,
        pub is_nft: bool,
        pub migrated_lock_deposit_id: ::ethers::core::types::U256,
        pub is_nft_minted: bool,
    }
    ///Container type for all return fields from the `getDepositsByWithdrawalAddress` function with signature `getDepositsByWithdrawalAddress(address)` and selector `0x0bd59ad3`
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
    pub struct GetDepositsByWithdrawalAddressReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getFeesInETH` function with signature `getFeesInETH(address)` and selector `0xfeeb733d`
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
    pub struct GetFeesInETHReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalTokenBalance` function with signature `getTotalTokenBalance(address)` and selector `0xadad19bd`
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
    pub struct GetTotalTokenBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isFreeToken` function with signature `isFreeToken(address)` and selector `0x31bff521`
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
    pub struct IsFreeTokenReturn(pub bool);
    ///Container type for all return fields from the `listMigratedDepositIds` function with signature `listMigratedDepositIds(uint256)` and selector `0x4d0925d3`
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
    pub struct ListMigratedDepositIdsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lockNFT` function with signature `lockNFT(address,address,uint256,uint256,uint256,bool,address)` and selector `0xe1b24aa2`
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
    pub struct LockNFTReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lockToken` function with signature `lockToken(address,address,uint256,uint256,bool,address)` and selector `0x5af06fed`
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
    pub struct LockTokenReturn {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lockedNFTs` function with signature `lockedNFTs(uint256)` and selector `0x945633c1`
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
    pub struct LockedNFTsReturn {
        pub token_address: ::ethers::core::types::Address,
        pub withdrawal_address: ::ethers::core::types::Address,
        pub token_amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
        pub withdrawn: bool,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lockedToken` function with signature `lockedToken(uint256)` and selector `0xbb941cff`
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
    pub struct LockedTokenReturn {
        pub token_address: ::ethers::core::types::Address,
        pub withdrawal_address: ::ethers::core::types::Address,
        pub token_amount: ::ethers::core::types::U256,
        pub unlock_time: ::ethers::core::types::U256,
        pub withdrawn: bool,
    }
    ///Container type for all return fields from the `nftMinted` function with signature `nftMinted(uint256)` and selector `0xffd68f15`
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
    pub struct NftMintedReturn(pub bool);
    ///Container type for all return fields from the `nonfungiblePositionManager` function with signature `nonfungiblePositionManager()` and selector `0xb44a2722`
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
    pub struct NonfungiblePositionManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub bool);
    ///Container type for all return fields from the `priceEstimator` function with signature `priceEstimator()` and selector `0xe3f1bc2b`
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
    pub struct PriceEstimatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `referralDiscount` function with signature `referralDiscount()` and selector `0x21721b17`
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
    pub struct ReferralDiscountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `referrerCut` function with signature `referrerCut()` and selector `0x4e1a5a7f`
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
    pub struct ReferrerCutReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `splitLock` function with signature `splitLock(uint256,uint256,uint256,bool)` and selector `0x047bcc70`
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
    pub struct SplitLockReturn {
        pub split_lock_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `usdTokenAddress` function with signature `usdTokenAddress()` and selector `0x3eac8dac`
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
    pub struct UsdTokenAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `v3Migrator` function with signature `v3Migrator()` and selector `0x6cbc1e9b`
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
    pub struct V3MigratorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `walletTokenBalance` function with signature `walletTokenBalance(address,address)` and selector `0xb9e7df1c`
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
    pub struct WalletTokenBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `whitelistAdmins` function with signature `whitelistAdmins(address)` and selector `0xd29dd76d`
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
    pub struct WhitelistAdminsReturn(pub bool);
    ///Container type for all return fields from the `whitelistedWallets` function with signature `whitelistedWallets(address)` and selector `0xa80dcfee`
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
    pub struct WhitelistedWalletsReturn(pub bool);
}
