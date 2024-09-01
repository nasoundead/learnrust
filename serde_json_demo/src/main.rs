use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}

#[cfg(test)]
mod tests {
    // Constructing JSON values
    // Serde JSON provides a json! macro to build serde_json::Value objects with very natural JSON syntax.
    use serde_json::json;

    #[test]
    fn constructing_json_values() {
	let full_name = "John Doe";
	let age_last_year = 42;
	// The type of `john` is `serde_json::Value`
	let john = json!(
	    {
	    "name": full_name,
	    "age": age_last_year + 1,
	    "phones": [
	    "+44 1234567",
	    "+44 2345678"
	    ]
	    }
	);
	println!("{}", john);
	println!("{}", john["age"]);
	println!("first phone number: {}", john["phones"][0]);
	// Convert to a string of JSON and print it out
	println!("{}", john.to_string());
    }

    // Operating on untyped JSON values
    // Any valid JSON data can be manipulated in the following recursive enum representation.
    // This data structure is serde_json::Value.

    // enum Value {
    //     Null,
    //     Bool(bool),
    //     Number(Number),
    //     String(String),
    //     Array(Vec<Value>),
    //     Object(Map<String, Value>),
    // }
    // A string of JSON data can be parsed into a serde_json::Value by the serde_json::from_str function.
    // There is also from_slice for parsing from a byte slice &[u8] and
    // from_reader for parsing from any io::Read like a File or a TCP stream.

    use serde_json::{Result, Value};

    #[test]
    fn untyped_example() -> Result<()> {
	// Some JSON input data as a &str. Maybe this comes from the user.
	let data = r#"
	    {
		"name": "John Doe",
		"age": 43,
		"phones": [
		    "+44 1234567",
		    "+44 2345678"
		]
	    }"#;

	// Parse the string of data into serde_json::Value.
	let v: Value = serde_json::from_str(data)?;

	// Access parts of the data by indexing with square brackets.
	println!("Please call {} at the number {}", v["name"], v["phones"][0]);

	Ok(())
    }

    // Parsing JSON as strongly typed data structures
    // Serde provides a powerful way of mapping JSON data into Rust data structures largely automatically.
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Person {
	name: String,
	age: u8,
	phones: Vec<String>,
    }

    #[test]
    fn typed_example() -> Result<()> {
	// Some JSON input data as a &str. Maybe this comes from the user.
	let data = r#"
	    {
		"name": "John Doe",
		"age": 43,
		"phones": [
		    "+44 1234567",
		    "+44 2345678"
		]
	    }"#;

	// Parse the string of data into a Person object. This is exactly the
	// same function as the one that produced serde_json::Value above, but
	// now we are asking it for a Person as output.
	let p: Person = serde_json::from_str(data)?;

	// Do things just like with any other Rust data structure.
	println!("Please call {} at the number {}", p.name, p.phones[0]);
	Ok(())
    }
}
