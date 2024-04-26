#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use js_sandbox::{AnyError, Script};

    use crate::get_timestamp;

    #[test]
    fn test_sc() -> Result<(), AnyError> {
        let src = r#"
function main(data) {
    var datac = JSON.parse(data).Data;
    var parse = JSON.parse(datac);
    var keyValuePairs = {};


    return [parse]
}
"#;


        let mut script = Script::from_string(src)?;

        let sample_data = r#"{"Timestamp":"1709102879000","Format":"JSON","Data":"{\"cherry\":94,\"orange\":100}"}"#;
        let c = "hello";
        let res: Vec<HashMap<String, u64>> = script.call("main", (sample_data, ))?;
        for item in res.iter() {
            for (key, value) in item.iter() {
                println!("Key: {}, Value: {}", key, value);
            }
        }
        Ok(())
    }


    #[test]
    fn test_sc2() -> Result<(), AnyError> {
        let src = r#"
        function main(data) {
            var datac = JSON.parse(data).Timestamp;
            return datac;
        }
    "#;

        let mut script = Script::from_string(src)?;

        let sample_data = r#"{"Timestamp":1709102879000,"Format":"JSON","Data":"{\"cherry\":94,\"orange\":100}"}"#;
        let res_str: f64 = script.call("main", (sample_data, ))?;

        println!("Parsed number: {}", res_str);
        let result = get_timestamp(src, sample_data);
        println!("{}", result.unwrap());
        Ok(())
    }
}