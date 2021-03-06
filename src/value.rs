/*! Rust interface to [sciter::value](https://github.com/c-smile/sciter-sdk/blob/master/include/value.h).

Sciter `Value` holds superset of JSON objects.

It can contain as pure JSON objects (numbers, strings, maps and arrays) as internal objects like DOM elements,
proxies of script functions, objects and arrays.


## Basic usage

You can create an empty (undefined) sciter `Value` with `new()`:

```
use sciter::Value;

let v = Value::new();
assert!(v.is_undefined());
assert!(!v.is_null());
```

Or explicitly create `Value` for specified type:

```
use sciter::Value;

let v = Value::null();
assert!(v.is_null());

let v = Value::symbol("hello");
assert!(v.is_symbol());
assert!(v.is_string());

let v = Value::error("hello");
assert!(v.is_error_string());
assert!(v.is_string());

let v = Value::array(4);
assert!(v.is_array());
assert!(v.len() == 4);

let v = Value::map();
assert!(v.is_map());
assert!(v.len() == 0);

```

Also there is conversion from Rust types:

```
use sciter::Value;

let v = Value::from(true);
assert!(v.is_bool());

let v = Value::from(1);
assert!(v.is_int());

let v = Value::from(1.0);
assert!(v.is_float());

let v = Value::from("hello");
assert!(v.is_string());

let v = Value::from(b"123".as_ref());
assert!(v.is_bytes());
```

And from sequence of objects:

```
use sciter::Value;

let v: Value = ["1","2","3"].iter().cloned().collect();
assert!(v.is_array());
assert_eq!(v.len(), 3);

assert_eq!(v[0], Value::from("1"));
```

To access its contents you should use one of `to_` methods:

```
use sciter::Value;

let v = Value::from(4);
assert_eq!(v.to_int(), Some(4));
```

Note that there is two functions that converts `Value` to JSON and back:

```
use sciter::Value;

let mut v: Value = "[1, 2, 3, 4]".parse().unwrap();
let json_str = v.into_string();
```

Array access:

```
use sciter::Value;

let mut v: Value = "[10, 20]".parse().unwrap();
assert_eq!(v[0], Value::from(10));

v.set(1, Value::from(21));
v.set(2, Value::from(22));
assert_eq!(v.len(), 3);

assert!(v.get(0).is_int());
```

Map access:

```
use sciter::Value;

let mut v: Value = "{one: 1, two: 2}".parse().unwrap();
assert_eq!(v[Value::from("one")], Value::from(1));

v.set_item(Value::from("three"), Value::from(3));
assert!(v.get_item(Value::from("one")).is_int());
```

.
*/

#![allow(dead_code)]

use ::{_API};
use capi::sctypes::*;
use capi::scvalue::{VALUE, VALUE_UNIT_TYPE_STRING};
pub use capi::scvalue::{VALUE_RESULT, VALUE_STRING_CVT_TYPE, VALUE_TYPE};

// TODO: map keys/values/items

/// sciter::value wrapper. See the module-level documentation.
pub struct Value
{
	data: VALUE,
	tmp: * mut Value,
}

impl Value {

	/// Return a new sciter value object ([undefined](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/undefined)).
	pub fn new() -> Value {
		Value { data: VALUE::default(), tmp: ::std::ptr::null_mut() }
	}

	/// Make explicit [array](http://sciter.com/docs/content/script/Array.htm) value with given length.
	pub fn array(length: usize) -> Value {
		let mut me = Value::new();
		(_API.ValueIntDataSet)(me.as_ptr(), length as i32, VALUE_TYPE::T_ARRAY as UINT, 0);
		return me;
	}

	/// Make explicit [map](http://sciter.com/docs/content/script/Object.htm) value.
	pub fn map() -> Value {
		let mut me = Value::new();
		(_API.ValueIntDataSet)(me.as_ptr(), 0i32, VALUE_TYPE::T_MAP as UINT, 0);
		return me;
	}

	/// Make explicit json [null](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/null) value.
	pub fn null() -> Value {
		let mut me = Value::new();
		me.data.t = VALUE_TYPE::T_NULL;
		return me;
	}

	/// Make sciter [symbol](http://sciter.com/docs/content/script/language/Syntax.htm#symbol-literals) value.
	pub fn symbol(val: &str) -> Value {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::SYMBOL);
		return me;
	}

	/// Make sciter [error](http://sciter.com/docs/content/script/Error.htm) value.
	pub fn error(val: &str) -> Value {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::ERROR);
		return me;
	}

	/// Parse json string into value.
	pub fn parse(val: &str) -> Result<Value, u32> {
		return Value::parse_as(val, VALUE_STRING_CVT_TYPE::JSON_LITERAL);
	}

	/// Parse json string into value.
	pub fn parse_as(val: &str, how: VALUE_STRING_CVT_TYPE) -> Result<Value, u32> {
		let mut me = Value::new();
		let (s,n) = s2w!(val);
		let ok: u32 = (_API.ValueFromString)(me.as_ptr(), s.as_ptr(), n, how);
		if ok == 0 {
			Ok(me)
		} else {
			Err(ok)
		}
	}

	/// Parse json string into value.
	/// Note that `Value::from_str()` parses a json string to value object and returns a `Result<Value>`
	/// unlike `Value::from()`, which returns just string object only.
	pub fn from_str(val: &str) -> Result<Self, VALUE_RESULT> {
		Value::parse(val).or(Err(VALUE_RESULT::BAD_PARAMETER))
	}

	#[doc(hidden)]
	pub fn as_ptr(&mut self) -> *mut VALUE {
		&mut self.data as *mut VALUE
	}

	#[doc(hidden)]
	pub fn as_cptr(&self) -> *const VALUE {
		&self.data as *const VALUE
	}

	#[doc(hidden)]
	pub fn as_mut_ptr(&self) -> * mut VALUE {
		unsafe { ::std::mem::transmute(self.as_cptr()) }
	}

	/// Get inner value type.
	pub fn get_type(&self) -> VALUE_TYPE {
		return self.data.t;
	}

	/// Get inner value type and its subtype (e.g. units).
	pub fn full_type(&self) -> (VALUE_TYPE, UINT) {
		return (self.data.t, self.data.u);
	}

	/// Convert T_OBJECT value type to JSON T_MAP or T_ARRAY.
	pub fn isolate(&mut self) {
		(_API.ValueIsolate)(self.as_ptr());
	}

	/// Clear the VALUE and deallocates all assosiated structures that are not used anywhere else.
	pub fn clear(&mut self) -> &mut Value {
		(_API.ValueClear)(self.as_ptr());
		self
	}

	/// Return the number of items in the T_ARRAY, T_MAP, T_FUNCTION and T_OBJECT sciter::value.
	pub fn len(&self) -> usize {
		let mut n: INT = 0;
		(_API.ValueElementsCount)(self.as_cptr(), &mut n);
		return n as usize;
	}

	/// Append value to the end of T_ARRAY sciter::value.
	pub fn push(&mut self, src: Value) {
		(_API.ValueNthElementValueSet)(self.as_ptr(), self.len() as INT, src.as_cptr());
	}

	/// Insert or set value at given `index` of T_ARRAY, T_MAP, T_FUNCTION and T_OBJECT sciter::value.
	pub fn set(&mut self, index: usize, src: Value) {
		(_API.ValueNthElementValueSet)(self.as_ptr(), index as INT, src.as_cptr());
	}

	/// Retreive value of sub-element at `index`
	///
	/// * T_ARRAY - nth element of the array;
	/// * T_MAP - value of nth key/value pair in the map;
	/// * T_FUNCTION - value of nth argument of the function.
	///
	pub fn get(&self, index: usize) -> Value {
		let mut v = Value::new();
		(_API.ValueNthElementValue)(self.as_cptr(), index as INT, v.as_ptr());
		return v;
	}

	/// Insert or set value of sub-element by key.
	///
	/// * if it is a map - sets named value in the map;
	/// * if it is a function - sets named argument of the function;
	/// * if it is a object - sets value of property of the object;
	/// * otherwise it converts this to map and adds key/v to it.
	///
	pub fn set_item(&mut self, key: Value, value: Value) {
		(_API.ValueSetValueToKey)(self.as_ptr(), key.as_cptr(), value.as_cptr());
	}

	/// Retrieve value of sub-element by key.
	pub fn get_item(&self, key: Value) -> Value {
		let mut v = Value::new();
		(_API.ValueGetValueOfKey)(self.as_cptr(), key.as_cptr(), v.as_ptr());
		return v;
	}

	/// Value to integer.
	pub fn to_int(&self) -> Option<i32> {
		let mut val = 0 as i32;
		match (_API.ValueIntData)(self.as_cptr(), &mut val) {
			VALUE_RESULT::OK => Some(val),
			_ => None
		}
	}

	/// Value to bool.
	pub fn to_bool(&self) -> Option<bool> {
		let mut val = 0 as i32;
		match (_API.ValueIntData)(self.as_cptr(), &mut val) {
			VALUE_RESULT::OK => Some(val != 0),
			_ => None
		}
	}

	/// Value to float.
	pub fn to_float(&self) -> Option<f64> {
		let mut val = 0 as f64;
		match (_API.ValueFloatData)(self.as_cptr(), &mut val) {
			VALUE_RESULT::OK => Some(val),
			_ => None
		}
	}

	/// Value as string for T_STRING type.
	pub fn as_string(&self) -> Option<String> {
		let mut s = 0 as LPCWSTR;
		let mut n = 0 as UINT;
		match (_API.ValueStringData)(self.as_cptr(), &mut s, &mut n) {
			VALUE_RESULT::OK => Some(::utf::w2sn(s, n as usize)),
			_ => None
		}
	}

	/// Value to json string (converted in-place). _Subject to change._
	pub fn into_string(&mut self) -> String {
		(_API.ValueToString)(self.as_ptr(), VALUE_STRING_CVT_TYPE::JSON_LITERAL);
		return self.as_string().unwrap();
	}

	/// Value as byte slice for T_BYTES type.
	pub fn as_bytes(&self) -> Option<&[u8]> {
		let mut s = 0 as LPCBYTE;
		let mut n = 0 as UINT;
		match (_API.ValueBinaryData)(self.as_cptr(), &mut s, &mut n) {
			VALUE_RESULT::OK => Some(unsafe { ::std::slice::from_raw_parts(s, n as usize) }),
			_ => None
		}
	}

	/// Value to byte vector for T_BYTES type.
	pub fn to_bytes(&self) -> Option<Vec<u8>> {
		match self.as_bytes() {
			Some(r) => Some(r.to_owned()),
			None => None,
		}
	}

	/// Function invocation for T_OBJECT/UT_OBJECT_FUNCTION.
	///
	/// Calls the tiscript function or method holded at `Value` with context of `this` object
	/// that will be known as _this_ inside that function (it is optional for global functions).
	/// `name` here is an url or name of the script - used for error reporting in the script.
	/// You can use the `make_args!(a,b,c)` macro which help you construct script arguments from Rust types.
	pub fn call(&self, this: Option<Value>, args: &[Value], name: Option<&str>) -> Result<Value, VALUE_RESULT> {
		let mut rv = Value::new();
		let argv = Value::pack_args(args);
		let (name,_) = s2w!(name.unwrap_or(""));
		let ok = (_API.ValueInvoke)(self.as_cptr(), this.unwrap_or(Value::default()).as_ptr(),
			argv.len() as UINT, argv.as_ptr(), rv.as_ptr(), name.as_ptr());
		match ok {
			VALUE_RESULT::OK => Ok(rv),
			_ => Err(ok)
		}
	}

	#[doc(hidden)]
	pub fn pack_to(&self, dst: &mut VALUE) {
		(_API.ValueCopy)(dst, self.as_cptr());
	}

	#[doc(hidden)]
	pub fn pack_args(args: &[Value]) -> Vec<VALUE> {
		let argc = args.len();
		let mut argv: Vec<VALUE> = Vec::with_capacity(argc);
		argv.resize(argc, VALUE::default());
		for i in 0..argc {
			args[i].pack_to(&mut argv[i]);
		}
		return argv;
	}

	#[doc(hidden)]
	pub fn unpack_from(args: * const VALUE, count: UINT) -> Vec<Value> {
		let argc = count as usize;
		let args = unsafe { ::std::slice::from_raw_parts(args, argc) };
		let mut argv: Vec<Value> = Vec::with_capacity(argc);
		for i in 0..argc {
			let mut v = Value::new();
			(_API.ValueCopy)(v.as_ptr(), &args[i]);
			argv.push(v);
		}
		return argv;
	}

	fn ensure_tmp_mut(&self) -> &mut Value {
		let cp = self as *const Value;
		let mp = cp as *mut Value;
		let me = unsafe { &mut *mp };
		return me.ensure_tmp();
	}

	fn ensure_tmp(&mut self) -> &mut Value {
		if self.tmp.is_null() {
			let tmp = Box::new(Value::new());
			self.tmp = Box::into_raw(tmp);
		}
		return unsafe { &mut *self.tmp };
	}

	// TODO: keys, values, items

	#[allow(missing_docs)]
	pub fn is_undefined(&self) -> bool {
		self.data.t == VALUE_TYPE::T_UNDEFINED
	}
	#[allow(missing_docs)]
	pub fn is_null(&self) -> bool {
		self.data.t == VALUE_TYPE::T_NULL
	}
	#[allow(missing_docs)]
	pub fn is_bool(&self) -> bool {
		self.data.t == VALUE_TYPE::T_BOOL
	}
	#[allow(missing_docs)]
	pub fn is_int(&self) -> bool {
		self.data.t == VALUE_TYPE::T_INT
	}
	#[allow(missing_docs)]
	pub fn is_float(&self) -> bool {
		self.data.t == VALUE_TYPE::T_FLOAT
	}
	#[allow(missing_docs)]
	pub fn is_bytes(&self) -> bool {
		self.data.t == VALUE_TYPE::T_BYTES
	}
	#[allow(missing_docs)]
	pub fn is_string(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING
	}
	#[allow(missing_docs)]
	pub fn is_symbol(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING && self.data.u == VALUE_UNIT_TYPE_STRING::SYMBOL as UINT
	}
	#[allow(missing_docs)]
	pub fn is_error_string(&self) -> bool {
		self.data.t == VALUE_TYPE::T_STRING && self.data.u == VALUE_UNIT_TYPE_STRING::ERROR as UINT
	}
	#[allow(missing_docs)]
	pub fn is_date(&self) -> bool {
		self.data.t == VALUE_TYPE::T_DATE
	}
	#[allow(missing_docs)]
	pub fn is_currency(&self) -> bool {
		self.data.t == VALUE_TYPE::T_CURRENCY
	}
	#[allow(missing_docs)]
	pub fn is_map(&self) -> bool {
		self.data.t == VALUE_TYPE::T_MAP
	}
	#[allow(missing_docs)]
	pub fn is_array(&self) -> bool {
		self.data.t == VALUE_TYPE::T_ARRAY
	}
	#[allow(missing_docs)]
	pub fn is_function(&self) -> bool {
		self.data.t == VALUE_TYPE::T_FUNCTION
	}
	#[allow(missing_docs)]
	pub fn is_native_function(&self) -> bool {
		(_API.ValueIsNativeFunctor)(self.as_cptr()) != 0
	}
	#[allow(missing_docs)]
	pub fn is_object(&self) -> bool {
		self.data.t == VALUE_TYPE::T_OBJECT
	}
	#[allow(missing_docs)]
	pub fn is_dom_element(&self) -> bool {
		self.data.t == VALUE_TYPE::T_DOM_OBJECT
	}

	fn assign_str(&mut self, val: &str, unit: VALUE_UNIT_TYPE_STRING) -> VALUE_RESULT {
		let (s,n) = s2w!(val);
		return (_API.ValueStringDataSet)(self.as_ptr(), s.as_ptr(), n, unit as UINT);
	}
}


/// Print `Value` as json string
impl ::std::fmt::Display for Value {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		let mut copy = self.clone();
		let re = copy.into_string();
		f.write_str(&re)
	}
}

/// Print `Value` as json string with explicit type showed.
impl ::std::fmt::Debug for Value {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
		let mut tname = format!("{:?}", self.data.t);
		if self.is_undefined() || self.is_null() {
			return f.write_str(&tname[2..].to_lowercase());
		}
		if self.is_string() && self.data.u != 0 {
			let units = ["symbol", "string", "error", "secure"];
			tname.push_str(":");
			tname.push_str(units[(self.data.u as i16 + 1) as usize]);
		}
		try!(f.write_str(&tname[2..].to_lowercase()));
		try!(f.write_str(":"));
		write!(f, "{}", &self)
	}
}

/// Destroy pointed value.
impl Drop for Value {
	fn drop(&mut self) {
		if !self.tmp.is_null() {
			unsafe { Box::from_raw(self.tmp) };
		}
		(_API.ValueClear)(self.as_ptr());
	}
}

/// Return default value (_undefined_).
impl Default for Value {
	fn default() -> Self {
		return Value::new();
	}
}

/// Copies value.
///
/// All allocated objects are reference counted so copying is just a matter of increasing reference counts.
impl Clone for Value {
	fn clone(&self) -> Self {
		let mut dst = Value::new();
		(_API.ValueCopy)(dst.as_ptr(), self.as_cptr());
		return dst;
	}
}

/// Compare two values.
impl ::std::cmp::PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match (_API.ValueCompare)(self.as_cptr(), other.as_cptr()) {
			VALUE_RESULT::OK_TRUE => true,
			VALUE_RESULT::OK => false,
			_ => false
		}
	}
}

/// Get item by index for array type.
impl ::std::ops::Index<usize> for Value {
	type Output = Value;
	fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueNthElementValue)(self.as_cptr(), index as INT, tmp.as_mut_ptr());
		return tmp;
	}
}

/// Set item by index for array type.
#[cfg(notworking)]
impl ::std::ops::IndexMut<usize> for Value {
	fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Value {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueNthElementValue)(self.as_cptr(), index as INT, tmp.as_ptr());
		return tmp;
	}
}

/// Get item by key for map type.
impl ::std::ops::Index<Value> for Value {
	type Output = Value;
	fn index<'a>(&'a self, key: Value) -> &'a Self::Output {
		let tmp = self.ensure_tmp_mut();
		(_API.ValueGetValueOfKey)(self.as_cptr(), key.as_cptr(), tmp.as_mut_ptr());
		return tmp;
	}
}

/// Set item by key for map type.
#[cfg(notworking)]
impl ::std::ops::IndexMut<Value> for Value {
	fn index_mut<'a>(&'a mut self, key: Value) -> &'a mut Value {
		let ptr = self.as_ptr();
		let tmp = self.ensure_tmp();
		(_API.ValueSetValueToKey)(ptr, key.as_cptr(), tmp.as_ptr());
		return tmp;
	}
}

/// Value from integer.
impl From<i32> for Value {
	// Note that there is no generic 64-bit integers at Sciter, only Date/Currency types.
	// There is a double (f64) for large numbers as workaround.
	fn from(val: i32) -> Self {
		let mut me = Value::new();
		(_API.ValueIntDataSet)(me.as_ptr(), val as i32, VALUE_TYPE::T_INT as UINT, 0);
		return me;
	}
}

/// Value from float.
impl From<f64> for Value {
	fn from(val: f64) -> Self {
		let mut me = Value::new();
		(_API.ValueFloatDataSet)(me.as_ptr(), val as f64, VALUE_TYPE::T_FLOAT as UINT, 0);
		return me;
	}
}

/// Value from bool.
impl From<bool> for Value {
	fn from(val: bool) -> Self {
		let mut me = Value::new();
		(_API.ValueIntDataSet)(me.as_ptr(), val as INT, VALUE_TYPE::T_BOOL as UINT, 0);
		return me;
	}
}

/// Value from string.
impl<'a> From<&'a str> for Value {
	fn from(val: &'a str) -> Self {
		let mut me = Value::new();
		me.assign_str(val, VALUE_UNIT_TYPE_STRING::STRING);
		return me;
	}
}

/// Value from string.
impl From<String> for Value {
	fn from(val: String) -> Self {
		let mut me = Value::new();
		me.assign_str(&val, VALUE_UNIT_TYPE_STRING::STRING);
		return me;
	}
}

/// Value from json string.
impl ::std::str::FromStr for Value {
	type Err = VALUE_RESULT;
	fn from_str(val: &str) -> Result<Self, Self::Err> {
		Value::from_str(val)
	}
}

/// Value from binary array (sequence of bytes).
impl<'a> From<&'a [u8]> for Value {
	fn from(val: &'a [u8]) -> Self {
		let mut me = Value::new();
		(_API.ValueBinaryDataSet)(me.as_ptr(), val.as_ptr(), val.len() as UINT, VALUE_TYPE::T_BYTES as UINT, 0);
		return me;
	}
}

/// Value from sequence of `i32`.
impl ::std::iter::FromIterator<i32> for Value {
	fn from_iter<I: IntoIterator<Item=i32>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from sequence of `f64`.
impl ::std::iter::FromIterator<f64> for Value {
	fn from_iter<I: IntoIterator<Item=f64>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from sequence of `&str`.
impl<'a> ::std::iter::FromIterator<&'a str> for Value {
	fn from_iter<I: IntoIterator<Item=&'a str>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from sequence of `String`.
impl ::std::iter::FromIterator<String> for Value {
	fn from_iter<I: IntoIterator<Item=String>>(iterator: I) -> Self {
		let mut v = Value::new();
		for i in iterator {
			v.push(Value::from(i));
		}
		return v;
	}
}

/// Value from function.
impl<F> From<F> for Value where F: Fn(&[Value]) -> Value {
	fn from(f: F) -> Value {
		let mut v = Value::new();
		let boxed = Box::new(f);
		let ptr = Box::into_raw(boxed);
		(_API.ValueNativeFunctorSet)(v.as_ptr(), _functor_invoke::<F>, _functor_release::<F>, ptr as LPVOID);
		return v;
	}
}

extern "C" fn _functor_release<F>(tag: LPVOID)
{
	// reconstruct handler from pointer
	let ptr = tag as *mut F;
	let boxed = unsafe { Box::from_raw(ptr) };
	// and forget it
	drop(boxed);
}

extern "C" fn _functor_invoke<F>(tag: LPVOID, argc: UINT, argv: *const VALUE, retval: *mut VALUE) where F: Fn(&[Value]) -> Value
{
	// reconstruct handler from pointer
	let ptr = tag as *mut F;
	let me = unsafe { &mut *ptr };
	let retval = unsafe { &mut *retval };
	let args = Value::unpack_from(argv, argc);
	let rv = me(&args);
	rv.pack_to(retval)
}


/// Helper trait
pub trait FromValue {
	/// Converts value to specified type.
	fn from_value(v: &Value) -> Option<Self> where Self: Sized;
}

impl FromValue for bool {
	fn from_value(v: &Value) -> Option<Self> {
		v.to_bool()
	}
}

impl FromValue for i32 {
	fn from_value(v: &Value) -> Option<Self> {
		v.to_int()
	}
}

impl FromValue for f64 {
	fn from_value(v: &Value) -> Option<Self> {
		v.to_float()
	}
}

impl FromValue for Vec<u8> {
	fn from_value(v: &Value) -> Option<Self> {
		v.to_bytes()
	}
}

impl FromValue for String {
	fn from_value(v: &Value) -> Option<Self> {
		v.as_string()
	}
}



mod tests {
	#![allow(unused_imports, unused_variables, unused_mut)]

	use super::{Value, FromValue};
	use capi::scvalue::*;
	use std::mem;
	use ::{_API};

	fn check1(a: i32) {
		assert_eq!(a, 12);
	}

	#[test]
	fn test_from_value() {
		let v = Value::from(12);
		check1(
			match FromValue::from_value(&v) {
				Some(x) => { x },
				None => { return; }
			}
		);
	}

	#[test]
	fn test_value_layout() {
		assert_eq!(mem::size_of::<VALUE_TYPE>(), 4);
		assert_eq!(mem::size_of::<VALUE>(), 16);
	}

	#[test]
	fn test_abi() {

		let mut data = VALUE { t: VALUE_TYPE::T_UNDEFINED, u: 0, d: 0 };
		assert_eq!(data.t, VALUE_TYPE::T_UNDEFINED);

		(_API.ValueInit)(&mut data);
		assert_eq!(data.t, VALUE_TYPE::T_UNDEFINED);

		(_API.ValueClear)(&mut data);
		assert_eq!(data.t, VALUE_TYPE::T_UNDEFINED);
	}
}
