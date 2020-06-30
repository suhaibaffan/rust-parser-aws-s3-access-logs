extern crate ferris_says;
extern crate regex;
use std::io::{ stdout, BufWriter};
use ferris_says::say;
use regex::Regex;
use std::collections::HashMap;
use rayon::prelude::*;

const I_AM_DONE_REGEX: &str = r#"[\S"\[\]]+|".*?"|\[.*?\]"#;

fn s3_parser( log :&str) -> Vec<HashMap<&str, regex::Captures<'_>>>{
    
    let re = Regex::new(I_AM_DONE_REGEX).unwrap();
    let mut finalResult = Vec::new(); 
    for (index, mat) in re.captures_iter(log ).enumerate() {
        let mut map = HashMap::new();
        let matchWord = mat;

        if index == 0 {
            map.insert("bucketOwner", matchWord );
        } else if index == 1 {
            map.insert("bucket", matchWord );
        } else if index == 2 {
            map.insert("time", matchWord );
        } else if index == 3 {
            map.insert("remoteIp", matchWord );
        } else if index == 4 {
            map.insert("requester", matchWord );
        } else if index == 5 {
            map.insert("requestId", matchWord );
        } else if index == 6 {
            map.insert("operation", matchWord );
        } else if index == 7 {
            map.insert("key", matchWord );
        } else if index == 8 {
            map.insert("requestUri", matchWord );
        } else if index == 9 {
            map.insert("httpStatus", matchWord );
        } else if index == 10 {
            map.insert("errorCode", matchWord );
        } else if index == 11 {
            map.insert("bytesSent", matchWord );
        } else if index == 12 {
            map.insert("objectSize", matchWord );
        } else if index == 13 {
            map.insert("totalTime", matchWord );
        } else if index == 14 {
            map.insert("turnArroundTime", matchWord );
        } else if index == 15 {
            map.insert("turnArroundTime", matchWord );
        }
        finalResult.push( map );
    }
    // println!("{:?}", finalResult );
    finalResult 
}

#[no_mangle]
pub extern fn parseLogs( logFile: String ) {
    let stdout = stdout();
    let message = String::from( "AWS_S3_Parser" );
    let width = message.chars().count();

    let mut writer = BufWriter::new( stdout.lock() );
    say( message.as_bytes(), width, &mut writer ).unwrap();

    let mut logs: Vec<_> = logFile.par_lines().map( |item| s3_parser( item ) ).collect();

    println!("{:?}", logs );
    // return parsedLogs;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn testParser() {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;
        use std::path::Path;

        let filePath = Path::new("./sample.txt").as_os_str();
        println!( "{:?}", filePath );
        let file = File::open( &filePath ).unwrap();
        let mut buf_reader = BufReader::new( file );
        let mut contents  = String::new();
        buf_reader.read_to_string( &mut contents ).unwrap();
        parseLogs( contents );
    }
}
