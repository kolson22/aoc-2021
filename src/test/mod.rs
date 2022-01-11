mod result;
use result::Error;

fn test() -> Result<(), Error> {
    "123".parse::<i32>;
    Ok()
}

fn main() {

}
