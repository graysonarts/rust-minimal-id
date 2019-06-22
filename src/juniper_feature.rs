use crate::MinimalId;
use juniper::Value;

juniper::graphql_scalar!(MinimalId {
	description: "A small, unique ID that works for most use-cases"

	resolve(&self) -> Value {
		Value::scalar(self.to_string())
	}

	from_input_value(v: &InputValue) -> Option<MinimalId> {
		v.as_scalar_value::<String>().map(|s| MinimalId::id_from_str(s).unwrap())
	}

	    from_str<'a>(value: ScalarToken<'a>) -> juniper::ParseScalarResult<'a, juniper::DefaultScalarValue> {
        <String as juniper::ParseScalarValue>::from_str(value)
    }
});

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn minimal_id_to_graphql() {}
}
