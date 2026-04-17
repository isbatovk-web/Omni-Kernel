//! # 🔤 Universal Type System
//!
//! The type system provides a language-agnostic representation of data types
//! that bridges the gap between different programming languages. When data
//! crosses module boundaries through the Bridge, it must be described using
//! these universal types.
//!
//! ## Type Mapping
//!
//! ```text
//! ┌────────────┬──────────┬──────────┬──────────┬──────────┐
//! │ Universal  │   Rust   │   C++    │    JS    │  Python  │
//! ├────────────┼──────────┼──────────┼──────────┼──────────┤
//! │ I32        │ i32      │ int32_t  │ number   │ int      │
//! │ I64        │ i64      │ int64_t  │ BigInt   │ int      │
//! │ F32        │ f32      │ float    │ number   │ float    │
//! │ F64        │ f64      │ double   │ number   │ float    │
//! │ Bool       │ bool     │ bool     │ boolean  │ bool     │
//! │ String     │ String   │ string   │ string   │ str      │
//! │ Bytes      │ Vec<u8>  │ vector   │ Uint8Arr │ bytes    │
//! │ List<T>    │ Vec<T>   │ vector<T>│ Array    │ list     │
//! │ Map<K,V>   │ HashMap  │ map      │ Map      │ dict     │
//! │ Option<T>  │ Option   │ optional │ null/val │ None/val │
//! │ Tuple(T..) │ (T..)    │ tuple    │ Array    │ tuple    │
//! │ Record{}   │ struct   │ struct   │ object   │ class    │
//! └────────────┴──────────┴──────────┴──────────┴──────────┘
//! ```

use std::collections::HashMap;
use thiserror::Error;

/// Errors in the type system.
#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Type mismatch: expected {expected}, got {actual}")]
    Mismatch { expected: String, actual: String },

    #[error("Unknown type: '{0}'")]
    UnknownType(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error at offset {offset}: {message}")]
    DeserializationError { offset: usize, message: String },

    #[error("Type '{0}' is not supported across the bridge")]
    UnsupportedType(String),

    #[error("Record field '{field}' not found in type '{record}'")]
    FieldNotFound { record: String, field: String },
}

/// Universal type representation for cross-language data exchange.
#[derive(Debug, Clone, PartialEq)]
pub enum UniversalType {
    // ── Primitive Types ────────────────────────────────────────
    /// Void / unit type.
    Void,
    /// Boolean value.
    Bool,
    /// 8-bit unsigned integer.
    U8,
    /// 16-bit unsigned integer.
    U16,
    /// 32-bit signed integer.
    I32,
    /// 64-bit signed integer.
    I64,
    /// 32-bit unsigned integer.
    U32,
    /// 64-bit unsigned integer.
    U64,
    /// 32-bit floating point.
    F32,
    /// 64-bit floating point.
    F64,

    // ── String & Binary ───────────────────────────────────────
    /// UTF-8 string.
    String,
    /// Raw byte array.
    Bytes,

    // ── Composite Types ───────────────────────────────────────
    /// Optional/nullable value.
    Option(Box<UniversalType>),
    /// Homogeneous list.
    List(Box<UniversalType>),
    /// Key-value map.
    Map(Box<UniversalType>, Box<UniversalType>),
    /// Fixed-size tuple of heterogeneous types.
    Tuple(Vec<UniversalType>),
    /// Named record (struct-like).
    Record(RecordType),
    /// Enum / variant type.
    Variant(VariantType),

    // ── Special Types ─────────────────────────────────────────
    /// Handle to a shared memory block (opaque pointer).
    MemHandle,
    /// Function reference (module + function name).
    FuncRef,
}

/// A record type (analogous to a struct).
#[derive(Debug, Clone, PartialEq)]
pub struct RecordType {
    /// Name of the record type.
    pub name: String,
    /// Fields and their types.
    pub fields: Vec<RecordField>,
}

/// A field within a record.
#[derive(Debug, Clone, PartialEq)]
pub struct RecordField {
    /// Field name.
    pub name: String,
    /// Field type.
    pub field_type: UniversalType,
    /// Whether this field is optional.
    pub optional: bool,
}

/// A variant/enum type.
#[derive(Debug, Clone, PartialEq)]
pub struct VariantType {
    /// Name of the variant type.
    pub name: String,
    /// Possible variants.
    pub variants: Vec<VariantCase>,
}

/// A single case of a variant type.
#[derive(Debug, Clone, PartialEq)]
pub struct VariantCase {
    /// Case name.
    pub name: String,
    /// Payload type (None if the case has no data).
    pub payload: Option<UniversalType>,
}

impl UniversalType {
    /// Get the size of this type in bytes when serialized.
    /// Returns None for variable-length types.
    pub fn fixed_size(&self) -> Option<usize> {
        match self {
            UniversalType::Void => Some(0),
            UniversalType::Bool | UniversalType::U8 => Some(1),
            UniversalType::U16 => Some(2),
            UniversalType::I32 | UniversalType::U32 | UniversalType::F32 => Some(4),
            UniversalType::I64 | UniversalType::U64 | UniversalType::F64 => Some(8),
            UniversalType::MemHandle => Some(4),
            _ => None, // Variable-length types
        }
    }

    /// Check if this type is a primitive (fixed-size, no nesting).
    pub fn is_primitive(&self) -> bool {
        self.fixed_size().is_some()
    }

    /// Get a human-readable type name.
    pub fn type_name(&self) -> String {
        match self {
            UniversalType::Void => "void".to_string(),
            UniversalType::Bool => "bool".to_string(),
            UniversalType::U8 => "u8".to_string(),
            UniversalType::U16 => "u16".to_string(),
            UniversalType::I32 => "i32".to_string(),
            UniversalType::I64 => "i64".to_string(),
            UniversalType::U32 => "u32".to_string(),
            UniversalType::U64 => "u64".to_string(),
            UniversalType::F32 => "f32".to_string(),
            UniversalType::F64 => "f64".to_string(),
            UniversalType::String => "string".to_string(),
            UniversalType::Bytes => "bytes".to_string(),
            UniversalType::Option(inner) => format!("option<{}>", inner.type_name()),
            UniversalType::List(inner) => format!("list<{}>", inner.type_name()),
            UniversalType::Map(k, v) => format!("map<{}, {}>", k.type_name(), v.type_name()),
            UniversalType::Tuple(types) => {
                let inner: Vec<_> = types.iter().map(|t| t.type_name()).collect();
                format!("tuple<{}>", inner.join(", "))
            }
            UniversalType::Record(r) => format!("record:{}", r.name),
            UniversalType::Variant(v) => format!("variant:{}", v.name),
            UniversalType::MemHandle => "mem-handle".to_string(),
            UniversalType::FuncRef => "func-ref".to_string(),
        }
    }
}

/// A concrete value in the universal type system.
#[derive(Debug, Clone)]
pub enum UniversalValue {
    Void,
    Bool(bool),
    U8(u8),
    U16(u16),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Bytes(Vec<u8>),
    Option(Option<Box<UniversalValue>>),
    List(Vec<UniversalValue>),
    Map(Vec<(UniversalValue, UniversalValue)>),
    Tuple(Vec<UniversalValue>),
    Record(HashMap<String, UniversalValue>),
    MemHandle(u32),
}

impl UniversalValue {
    /// Serialize a value to bytes (little-endian wire format).
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            UniversalValue::Void => vec![],
            UniversalValue::Bool(v) => vec![if *v { 1 } else { 0 }],
            UniversalValue::U8(v) => vec![*v],
            UniversalValue::U16(v) => v.to_le_bytes().to_vec(),
            UniversalValue::I32(v) => v.to_le_bytes().to_vec(),
            UniversalValue::I64(v) => v.to_le_bytes().to_vec(),
            UniversalValue::U32(v) => v.to_le_bytes().to_vec(),
            UniversalValue::U64(v) => v.to_le_bytes().to_vec(),
            UniversalValue::F32(v) => v.to_le_bytes().to_vec(),
            UniversalValue::F64(v) => v.to_le_bytes().to_vec(),
            UniversalValue::String(s) => {
                let bytes = s.as_bytes();
                let len = bytes.len() as u32;
                let mut result = len.to_le_bytes().to_vec();
                result.extend_from_slice(bytes);
                result
            }
            UniversalValue::Bytes(b) => {
                let len = b.len() as u32;
                let mut result = len.to_le_bytes().to_vec();
                result.extend_from_slice(b);
                result
            }
            UniversalValue::Option(opt) => match opt {
                None => vec![0],
                Some(val) => {
                    let mut result = vec![1];
                    result.extend(val.serialize());
                    result
                }
            },
            UniversalValue::List(items) => {
                let len = items.len() as u32;
                let mut result = len.to_le_bytes().to_vec();
                for item in items {
                    let serialized = item.serialize();
                    let item_len = serialized.len() as u32;
                    result.extend(item_len.to_le_bytes());
                    result.extend(serialized);
                }
                result
            }
            UniversalValue::Tuple(items) => {
                let mut result = Vec::new();
                for item in items {
                    result.extend(item.serialize());
                }
                result
            }
            UniversalValue::MemHandle(h) => h.to_le_bytes().to_vec(),
            UniversalValue::Map(entries) => {
                let len = entries.len() as u32;
                let mut result = len.to_le_bytes().to_vec();
                for (key, value) in entries {
                    let k = key.serialize();
                    let v = value.serialize();
                    result.extend((k.len() as u32).to_le_bytes());
                    result.extend(k);
                    result.extend((v.len() as u32).to_le_bytes());
                    result.extend(v);
                }
                result
            }
            UniversalValue::Record(fields) => {
                let len = fields.len() as u32;
                let mut result = len.to_le_bytes().to_vec();
                for (name, value) in fields {
                    // Serialize field name
                    let name_bytes = name.as_bytes();
                    result.extend((name_bytes.len() as u32).to_le_bytes());
                    result.extend(name_bytes);
                    // Serialize field value
                    let v = value.serialize();
                    result.extend((v.len() as u32).to_le_bytes());
                    result.extend(v);
                }
                result
            }
        }
    }

    /// Get the type of this value.
    pub fn get_type(&self) -> UniversalType {
        match self {
            UniversalValue::Void => UniversalType::Void,
            UniversalValue::Bool(_) => UniversalType::Bool,
            UniversalValue::U8(_) => UniversalType::U8,
            UniversalValue::U16(_) => UniversalType::U16,
            UniversalValue::I32(_) => UniversalType::I32,
            UniversalValue::I64(_) => UniversalType::I64,
            UniversalValue::U32(_) => UniversalType::U32,
            UniversalValue::U64(_) => UniversalType::U64,
            UniversalValue::F32(_) => UniversalType::F32,
            UniversalValue::F64(_) => UniversalType::F64,
            UniversalValue::String(_) => UniversalType::String,
            UniversalValue::Bytes(_) => UniversalType::Bytes,
            UniversalValue::Option(_) => UniversalType::Option(Box::new(UniversalType::Void)),
            UniversalValue::List(_) => UniversalType::List(Box::new(UniversalType::Void)),
            UniversalValue::Map(_) => UniversalType::Map(
                Box::new(UniversalType::Void),
                Box::new(UniversalType::Void),
            ),
            UniversalValue::Tuple(items) => {
                UniversalType::Tuple(items.iter().map(|i| i.get_type()).collect())
            }
            UniversalValue::Record(_) => UniversalType::Record(RecordType {
                name: "anonymous".to_string(),
                fields: Vec::new(),
            }),
            UniversalValue::MemHandle(_) => UniversalType::MemHandle,
        }
    }

    /// Try to extract an i32 value.
    pub fn as_i32(&self) -> Result<i32, TypeError> {
        match self {
            UniversalValue::I32(v) => Ok(*v),
            _ => Err(TypeError::Mismatch {
                expected: "i32".to_string(),
                actual: format!("{:?}", self.get_type()),
            }),
        }
    }

    /// Try to extract an i64 value.
    pub fn as_i64(&self) -> Result<i64, TypeError> {
        match self {
            UniversalValue::I64(v) => Ok(*v),
            _ => Err(TypeError::Mismatch {
                expected: "i64".to_string(),
                actual: format!("{:?}", self.get_type()),
            }),
        }
    }

    /// Try to extract a string value.
    pub fn as_string(&self) -> Result<&str, TypeError> {
        match self {
            UniversalValue::String(v) => Ok(v.as_str()),
            _ => Err(TypeError::Mismatch {
                expected: "string".to_string(),
                actual: format!("{:?}", self.get_type()),
            }),
        }
    }

    /// Try to extract a bool value.
    pub fn as_bool(&self) -> Result<bool, TypeError> {
        match self {
            UniversalValue::Bool(v) => Ok(*v),
            _ => Err(TypeError::Mismatch {
                expected: "bool".to_string(),
                actual: format!("{:?}", self.get_type()),
            }),
        }
    }

    /// Try to extract an f64 value.
    pub fn as_f64(&self) -> Result<f64, TypeError> {
        match self {
            UniversalValue::F64(v) => Ok(*v),
            UniversalValue::F32(v) => Ok(*v as f64), // Auto-widen
            _ => Err(TypeError::Mismatch {
                expected: "f64".to_string(),
                actual: format!("{:?}", self.get_type()),
            }),
        }
    }
}

/// Function signature in the universal type system.
#[derive(Debug, Clone)]
pub struct FunctionType {
    /// Function name.
    pub name: String,
    /// Parameter types.
    pub params: Vec<(String, UniversalType)>,
    /// Return type.
    pub returns: UniversalType,
}

impl FunctionType {
    /// Check if a set of arguments matches this function's parameter types.
    pub fn validate_args(&self, args: &[UniversalValue]) -> Result<(), TypeError> {
        if args.len() != self.params.len() {
            return Err(TypeError::Mismatch {
                expected: format!("{} arguments", self.params.len()),
                actual: format!("{} arguments", args.len()),
            });
        }

        for (i, (param_name, param_type)) in self.params.iter().enumerate() {
            let arg_type = args[i].get_type();
            if arg_type != *param_type && !Self::is_compatible(&arg_type, param_type) {
                return Err(TypeError::Mismatch {
                    expected: format!("{}:{}", param_name, param_type.type_name()),
                    actual: arg_type.type_name(),
                });
            }
        }

        Ok(())
    }

    /// Check if two types are compatible (allows implicit widening).
    fn is_compatible(actual: &UniversalType, expected: &UniversalType) -> bool {
        match (actual, expected) {
            // Integer widening
            (UniversalType::I32, UniversalType::I64) => true,
            (UniversalType::U8, UniversalType::I32) => true,
            (UniversalType::U8, UniversalType::I64) => true,
            (UniversalType::U16, UniversalType::I32) => true,
            (UniversalType::U16, UniversalType::I64) => true,
            (UniversalType::U32, UniversalType::U64) => true,
            // Float widening
            (UniversalType::F32, UniversalType::F64) => true,
            // Same type is always compatible
            _ => actual == expected,
        }
    }
}

/// Type registry — maps type names to their definitions.
pub struct TypeRegistry {
    /// Registered record types.
    records: HashMap<String, RecordType>,
    /// Registered variant types.
    variants: HashMap<String, VariantType>,
    /// Registered function types.
    functions: HashMap<(String, String), FunctionType>, // (module, func_name)
}

impl TypeRegistry {
    /// Create a new type registry.
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            variants: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Register a record type.
    pub fn register_record(&mut self, record: RecordType) {
        self.records.insert(record.name.clone(), record);
    }

    /// Register a variant type.
    pub fn register_variant(&mut self, variant: VariantType) {
        self.variants.insert(variant.name.clone(), variant);
    }

    /// Register a function type for a module.
    pub fn register_function(&mut self, module: &str, func_type: FunctionType) {
        let key = (module.to_string(), func_type.name.clone());
        self.functions.insert(key, func_type);
    }

    /// Look up a record type by name.
    pub fn get_record(&self, name: &str) -> Option<&RecordType> {
        self.records.get(name)
    }

    /// Look up a function type.
    pub fn get_function(&self, module: &str, func_name: &str) -> Option<&FunctionType> {
        let key = (module.to_string(), func_name.to_string());
        self.functions.get(&key)
    }

    /// Validate a cross-module function call with type checking.
    pub fn validate_call(
        &self,
        module: &str,
        func_name: &str,
        args: &[UniversalValue],
    ) -> Result<(), TypeError> {
        let func_type = self.get_function(module, func_name).ok_or_else(|| {
            TypeError::UnknownType(format!("{}::{}", module, func_name))
        })?;

        func_type.validate_args(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip_primitives() {
        let values = vec![
            UniversalValue::I32(42),
            UniversalValue::I64(-1234567890),
            UniversalValue::F64(3.14159),
            UniversalValue::Bool(true),
        ];

        for val in &values {
            let bytes = val.serialize();
            assert!(!bytes.is_empty());
        }
    }

    #[test]
    fn test_string_serialization() {
        let val = UniversalValue::String("Hello, Omni-Kernel!".to_string());
        let bytes = val.serialize();

        // First 4 bytes should be length
        let len = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        assert_eq!(len, 19);

        let string = std::str::from_utf8(&bytes[4..]).unwrap();
        assert_eq!(string, "Hello, Omni-Kernel!");
    }

    #[test]
    fn test_type_compatibility() {
        let func = FunctionType {
            name: "process".to_string(),
            params: vec![
                ("x".to_string(), UniversalType::I64),
                ("y".to_string(), UniversalType::F64),
            ],
            returns: UniversalType::F64,
        };

        // Exact match
        let args = vec![
            UniversalValue::I64(42),
            UniversalValue::F64(3.14),
        ];
        assert!(func.validate_args(&args).is_ok());

        // Int widening: i32 → i64 should work
        let args_widened = vec![
            UniversalValue::I32(42), // i32 → i64 implicit widen
            UniversalValue::F64(3.14),
        ];
        assert!(func.validate_args(&args_widened).is_ok());

        // Wrong type
        let args_wrong = vec![
            UniversalValue::String("oops".to_string()),
            UniversalValue::F64(3.14),
        ];
        assert!(func.validate_args(&args_wrong).is_err());
    }

    #[test]
    fn test_type_names() {
        assert_eq!(UniversalType::I32.type_name(), "i32");
        assert_eq!(
            UniversalType::List(Box::new(UniversalType::String)).type_name(),
            "list<string>"
        );
        assert_eq!(
            UniversalType::Map(
                Box::new(UniversalType::String),
                Box::new(UniversalType::I64)
            )
            .type_name(),
            "map<string, i64>"
        );
        assert_eq!(
            UniversalType::Option(Box::new(UniversalType::F64)).type_name(),
            "option<f64>"
        );
    }

    #[test]
    fn test_record_type() {
        let point_type = RecordType {
            name: "Point3D".to_string(),
            fields: vec![
                RecordField {
                    name: "x".to_string(),
                    field_type: UniversalType::F64,
                    optional: false,
                },
                RecordField {
                    name: "y".to_string(),
                    field_type: UniversalType::F64,
                    optional: false,
                },
                RecordField {
                    name: "z".to_string(),
                    field_type: UniversalType::F64,
                    optional: false,
                },
            ],
        };

        assert_eq!(point_type.fields.len(), 3);
        assert_eq!(
            UniversalType::Record(point_type).type_name(),
            "record:Point3D"
        );
    }

    #[test]
    fn test_value_extraction() {
        let val = UniversalValue::I32(42);
        assert_eq!(val.as_i32().unwrap(), 42);
        assert!(val.as_string().is_err());

        let sval = UniversalValue::String("hello".to_string());
        assert_eq!(sval.as_string().unwrap(), "hello");
        assert!(sval.as_i32().is_err());
    }
}
