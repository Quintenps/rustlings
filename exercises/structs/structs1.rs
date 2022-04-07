// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    name: &'static str,
    hex: &'static str
}

struct ColorTupleStruct(&'static str, &'static str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;
    
    // String::from("green")
    // String::from("#00FF00")

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let mut green = ColorClassicStruct {
            name: "green",
            hex: "#00FF00"
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");

        let other_green = &green;

        assert_eq!(other_green.name, "green");
        assert_eq!(other_green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
