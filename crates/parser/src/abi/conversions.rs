use starknet::core::types::contract::{
    AbiEnum, AbiEventEnum, AbiEventStruct, AbiStruct, EventFieldKind,
    StateMutability as StarknetStateMutability,
};

use crate::tokens::{CompositeInner, CompositeInnerKind, CompositeType, StateMutability, Token};
use crate::Error;

impl From<StarknetStateMutability> for StateMutability {
    fn from(value: StarknetStateMutability) -> Self {
        match value {
            StarknetStateMutability::External => StateMutability::External,
            StarknetStateMutability::View => StateMutability::View,
        }
    }
}

impl From<EventFieldKind> for CompositeInnerKind {
    fn from(value: EventFieldKind) -> Self {
        match value {
            EventFieldKind::Key => CompositeInnerKind::Key,
            EventFieldKind::Data => CompositeInnerKind::Data,
            EventFieldKind::Nested => CompositeInnerKind::Nested,
            EventFieldKind::Flat => CompositeInnerKind::Flat,
        }
    }
}

impl TryFrom<&AbiStruct> for Token {
    type Error = Error;

    fn try_from(value: &AbiStruct) -> Result<Self, Self::Error> {
        let mut t = Token::parse(&value.name)?;

        if let Token::Composite(ref mut c) = t {
            c.r#type = CompositeType::Struct;

            for (i, m) in value.members.iter().enumerate() {
                c.inners.push(CompositeInner {
                    index: i,
                    name: m.name.clone(),
                    token: Token::parse(&m.r#type).unwrap(),
                    kind: CompositeInnerKind::NotUsed,
                });
            }

            if !c.generic_args.is_empty() {
                let mut token = Token::Composite(c.clone());
                for (g_name, g_token) in c.generic_args.iter() {
                    token = token.resolve_generic(g_name, &g_token.type_path());
                }

                return Ok(token);
            }

            Ok(t)
        } else {
            Err(Error::ParsingFailed(format!(
                "AbiStruct is expected to be a Composite token, got `{:?}`",
                value,
            )))
        }
    }
}

impl TryFrom<&AbiEnum> for Token {
    type Error = Error;

    fn try_from(value: &AbiEnum) -> Result<Self, Self::Error> {
        let mut t = Token::parse(&value.name)?;

        if let Token::Composite(ref mut c) = t {
            c.r#type = CompositeType::Enum;

            for (i, v) in value.variants.iter().enumerate() {
                c.inners.push(CompositeInner {
                    index: i,
                    name: v.name.clone(),
                    token: Token::parse(&v.r#type).unwrap(),
                    kind: CompositeInnerKind::NotUsed,
                });
            }

            if !c.generic_args.is_empty() {
                let mut token = Token::Composite(c.clone());
                for (g_name, g_token) in c.generic_args.iter() {
                    token = token.resolve_generic(g_name, &g_token.type_path());
                }

                return Ok(token);
            }

            Ok(t)
        } else {
            Err(Error::ParsingFailed(format!(
                "AbiEnum is expected to be a Composite token, got `{:?}`",
                value,
            )))
        }
    }
}

impl TryFrom<&AbiEventStruct> for Token {
    type Error = Error;

    fn try_from(value: &AbiEventStruct) -> Result<Self, Self::Error> {
        let mut t = Token::parse(&value.name)?;

        if let Token::Composite(ref mut c) = t {
            c.r#type = CompositeType::Struct;
            c.is_event = true;

            for (i, m) in value.members.iter().enumerate() {
                c.inners.push(CompositeInner {
                    index: i,
                    name: m.name.clone(),
                    token: Token::parse(&m.r#type).unwrap(),
                    kind: m.kind.clone().into(),
                });
            }

            if !c.generic_args.is_empty() {
                let mut token = Token::Composite(c.clone());
                for (g_name, g_token) in c.generic_args.iter() {
                    token = token.resolve_generic(g_name, &g_token.type_path());
                }

                return Ok(token);
            }

            Ok(t)
        } else {
            Err(Error::ParsingFailed(format!(
                "AbiEventStruct is expected to be a Composite token, got `{:?}`",
                value,
            )))
        }
    }
}

impl TryFrom<&AbiEventEnum> for Token {
    type Error = Error;

    fn try_from(value: &AbiEventEnum) -> Result<Self, Self::Error> {
        let mut t = Token::parse(&value.name)?;

        if let Token::Composite(ref mut c) = t {
            c.r#type = CompositeType::Enum;
            c.is_event = true;

            for (i, v) in value.variants.iter().enumerate() {
                c.inners.push(CompositeInner {
                    index: i,
                    name: v.name.clone(),
                    token: Token::parse(&v.r#type).unwrap(),
                    kind: v.kind.clone().into(),
                });
            }

            if !c.generic_args.is_empty() {
                let mut token = Token::Composite(c.clone());
                for (g_name, g_token) in c.generic_args.iter() {
                    token = token.resolve_generic(g_name, &g_token.type_path());
                }

                return Ok(token);
            }

            Ok(t)
        } else {
            Err(Error::ParsingFailed(format!(
                "AbiEventEnum is expected to be a Composite token, got `{:?}`",
                value,
            )))
        }
    }
}
